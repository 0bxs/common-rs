use core::str;
use std::collections::{BTreeMap, HashMap};
use std::time::{SystemTime, SystemTimeError};
use chrono::DateTime;
use hmac::{Hmac, Mac};
use percent_encoding::{NON_ALPHANUMERIC, utf8_percent_encode};
use serde_json::{json, Value};
use std::borrow::Cow;
use reqwest::{
    Client,
    header::{HeaderMap, HeaderValue}, Method, Response, StatusCode,
};
use sha2::{Digest, Sha256};
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use sha2::digest::KeyInit;
use crate::utils::request::client;

// 生成 x-acs-date
pub fn current_timestamp() -> Result<u64, SystemTimeError> {
    Ok(SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)?
        .as_secs())
}
// URL编码处理
pub fn percent_code(encode_str: &str) -> Cow<'_, str> {
    let encoded = utf8_percent_encode(encode_str, NON_ALPHANUMERIC)
        .to_string()
        .replace("+", "20%")
        .replace("%5F", "_")
        .replace("%2D", "-")
        .replace("%2E", ".")
        .replace("%7E", "~");

    Cow::Owned(encoded) // 返回一个 Cow<str> 可以持有 String 或 &str
}

fn flatten_target_ops(
    targets: Vec<HashMap<&str, &str>>,
    base_key: &str,
) -> Vec<(&'static str, &'static str)> {
    let mut result = Vec::new();

    for (idx, item) in targets.iter().enumerate() {
        let prefix = format!("{}.{}", base_key, idx + 1);

        for (&k, &v) in item {
            let key = format!("{}.{}", prefix, k);
            let key_static: &'static str = Box::leak(key.into_boxed_str());
            let value_static: &'static str = Box::leak(v.to_string().into_boxed_str());

            result.push((key_static, value_static));
        }
    }

    result
}

/// 计算SHA256哈希
pub fn sha256_hex(message: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(message.as_bytes());
    hex::encode(hasher.finalize()) // ✅ 64 位小写 hex
}

// HMAC SHA256
pub fn hmac256(key: &[u8], message: &str) -> Result<Vec<u8>, String> {
    let mut mac = Hmac::<Sha256>::new_from_slice(key)
        .map_err(|e| format!("use data key on sha256 fail:{}", e))?;
    mac.update(message.as_bytes());
    let signature = mac.finalize();
    Ok(signature.into_bytes().to_vec())
}
// 生成签名唯一随机数
pub fn generate_random_string(length: usize) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";
    (0..length)
        .map(|_| CHARSET[rand::random_range(0..CHARSET.len())] as char)
        .collect()
}
pub fn generate_nonce() -> String {
    generate_random_string(32)
}
// 构建规范化查询参数（编码后）
pub fn build_sored_encoded_query_string(query_params: &[(&str, &str)]) -> String {
    let sorted_query_params: BTreeMap<_, _> = query_params.iter().copied().collect();
    let encoded_params: Vec<String> = sorted_query_params
        .into_iter()
        .map(|(k, v)| {
            let encoded_key = percent_code(k);
            let encoded_value = percent_code(v);
            format!("{}={}", encoded_key, encoded_value)
        })
        .collect();
    encoded_params.join("&")
}
// 读取响应
pub async fn read_response(result: Response) -> Result<(StatusCode, String), String> {
    let status = result.status();
    let data = result.bytes().await.map_err(|e| format!("Read response body failed: {}", e))?;
    let res = match str::from_utf8(&data) {
        Ok(s) => s.to_string(),
        Err(_) => return Err("Body contains non UTF-8 characters".to_string()),
    };
    Ok((status, res))
}
// 定义 FormData 类型数据的value类型
#[derive(Debug, Clone)]
pub enum FormValue {
    String(String),
    Vec(Vec<String>),
    HashMap(HashMap<String, String>),
}
// 定义一个body请求体枚举，用于统一处理请求体类型,包含Json/Binary/FormData类型
pub enum RequestBody {
    Json(HashMap<String, Value>), // Json
    Binary(Vec<u8>), // Binary
    FormData(HashMap<String, FormValue>), //  FormData
    None,
}
// 规范化请求
pub async fn call_api(
    method: Method,
    host: &str,
    canonical_uri: &str,
    query_params: &[(&str, &str)],
    action: &str,
    version: &str,
    body: RequestBody,
    access_key_id: &str,
    access_key_secret: &str,
) -> Result<String, String> {

    // 根据 body 类型处理请求体内容,将处理后的存储在 body_content 变量中。
    let body_content = match &body {
        RequestBody::Json(body_map) => json!(body_map).to_string(),
        RequestBody::Binary(binary_data) => {
            STANDARD.encode(binary_data)
        }
        RequestBody::FormData(form_data) => {
            let params: Vec<String> = form_data
                .iter()
                .flat_map(|(k, v)| {
                    match v {
                        FormValue::String(s) => {
                            vec![format!("{}={}", percent_code(k), percent_code(&s))]
                        }
                        FormValue::Vec(vec) => {
                            vec.iter()
                                .map(|s| format!("{}={}", percent_code(k), percent_code(s)))
                                .collect::<Vec<_>>()
                        }
                        FormValue::HashMap(map) => {
                            map.iter()
                                .map(|(sk, sv)| format!("{}={}", percent_code(sk), percent_code(sv)))
                                .collect::<Vec<_>>()
                        }
                    }
                })
                .collect();
            params.join("&")
        }
        RequestBody::None => String::new(),
    };

    // 计算 请求体body的x-acs-content-sha256 ；准备x-acs-date； x-acs-signature-nonce；待签名请求头
    let hashed_request_payload = if body_content.is_empty() {
        sha256_hex("")
    } else {
        sha256_hex(&body_content)
    };
    // x-acs-date
    let now_time = current_timestamp().map_err(|e| format!("Get current timestamp failed: {}", e))?;
    let datetime = DateTime::from_timestamp(now_time as i64, 0).ok_or_else(|| format!("Get datetime from timestamp failed: {}", now_time))?;
    let datetime_str = datetime.format("%Y-%m-%dT%H:%M:%SZ").to_string();
    // x-acs-signature-nonce
    let signature_nonce = generate_nonce();
    println!("Signature Nonce: {}", signature_nonce);
    // 待签名请求头
    let sign_header_arr = &[
        "host",
        "x-acs-action",
        "x-acs-content-sha256",
        "x-acs-date",
        "x-acs-signature-nonce",
        "x-acs-version",
    ];
    let sign_headers = sign_header_arr.join(";");
    // 1.构造规范化请求头
    let mut headers = HeaderMap::new();
    headers.insert("Host", HeaderValue::from_str(host).unwrap());
    headers.insert("x-acs-action", HeaderValue::from_str(action).unwrap());
    headers.insert("x-acs-version", HeaderValue::from_str(version).unwrap());
    headers.insert("x-acs-date", HeaderValue::from_str(&datetime_str).unwrap());
    headers.insert("x-acs-signature-nonce", HeaderValue::from_str(&signature_nonce).unwrap());
    headers.insert("x-acs-content-sha256", HeaderValue::from_str(&hashed_request_payload).unwrap());
    // 2.构造待签名请求头
    let canonical_query_string = build_sored_encoded_query_string(query_params); // 参数编码拼接处理
    println!("CanonicalQueryString: {}", canonical_query_string);
    let canonical_request = format!(
        "{}\n{}\n{}\n{}\n\n{}\n{}",
        method.as_str(),
        canonical_uri,
        canonical_query_string,
        sign_header_arr.iter().map(|&header| format!("{}:{}", header, headers[header].to_str().unwrap())).collect::<Vec<_>>().join("\n"),
        sign_headers,
        hashed_request_payload
    );
    println!("Canonical Request: {}", canonical_request);
    // 3.计算待签名请求头的 SHA-256 哈希;
    let result = sha256_hex(&canonical_request);
    // 4.构建待签名字符串
    let string_to_sign = format!("ACS3-HMAC-SHA256\n{}", result);
    // 5.计算签名
    let signature = hmac256(access_key_secret.as_bytes(), &string_to_sign)?;
    let data_sign = hex::encode(&signature);
    let auth_data = format!(
        "ACS3-HMAC-SHA256 Credential={},SignedHeaders={},Signature={}",
        access_key_id, sign_headers, data_sign
    );
    // 6.构建 Authorization
    headers.insert("Authorization", HeaderValue::from_str(&auth_data).unwrap());
    // 构造 url 拼接请求参数
    let url: String;
    if !query_params.is_empty() {
        url = format!("https://{}{}?{}", host, canonical_uri, canonical_query_string);
    } else {
        url = format!("https://{}{}", host, canonical_uri);
    }
    // 调用发送请求
    let response = send_request(
        method,
        &url,
        headers,
        query_params,
        &body,
        &body_content,
    )
        .await?;

    // 读取响应
    let (_, res) = read_response(response).await?;
    Ok(res)
}

/// 发送请求
async fn send_request(
    method: Method,
    url: &str,
    headers: HeaderMap,
    query_params: &[(&str, &str)],     // 接收 query 参数
    body: &RequestBody,                // 用此判断 body 数据类型
    body_content: &str,                // body 不为空时 接收 body 请求参数 FormData/Json/Binary
) -> Result<Response, String> {
    let mut request_builder = client().request(method.clone(), url);
    // 添加请求头 headers
    for (k, v) in headers.iter() {
        request_builder = request_builder.header(k, v.clone());
    }
    // 添加请求体 body
    match body {
        RequestBody::Binary(_) => {
            request_builder = request_builder.header("Content-Type", "application/octet-stream");
            request_builder = request_builder.body(body_content.to_string()); // 移动这里的值
        }
        RequestBody::Json(_) => {
            // 如果body为map，且不为空，转化为Json后存储在 body_content 变量中，设置  application/json; charset=utf-8
            if !body_content.is_empty() {
                request_builder = request_builder.body(body_content.to_string());
                request_builder = request_builder.header("Content-Type", "application/json; charset=utf-8");
            }
        }
        RequestBody::FormData(_) => {
            // 处理 form-data 类型，设置 content-type
            if !body_content.is_empty() {
                request_builder = request_builder.header("Content-Type", "application/x-www-form-urlencoded");
                request_builder = request_builder.body(body_content.to_string());
            }
        }
        RequestBody::None => {
            request_builder = request_builder.body(String::new());
        }
    }
    // 构建请求
    let req = request_builder
        .build()
        .map_err(|e| format!("build request fail: {}", e))?;
    // 发起请求
    let response = client()
        .execute(req)
        .await
        .map_err(|e| format!("execute request fail: {}", e))?;
    // 返回结果
    Ok(response)
}