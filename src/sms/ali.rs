use chrono::Utc;
use hmac::{Hmac, Mac};
use reqwest::{Client, Method};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, HashMap};
use std::env;
use std::sync::OnceLock;
use crate::sms::ali_req::{call_api, RequestBody};
use crate::utils::request::client;

#[derive(Debug)]
pub struct Conf {
    pub access_key: String,
    pub secret_key: String,
    pub sign_name: String,
    pub templates: HashMap<i8, Template>,
}

#[derive(Debug)]
pub struct Template {
    pub code: String,
    pub param: String,
}

static CONF: OnceLock<Conf> = OnceLock::new();

pub fn init(conf: Conf) {
    CONF.set(conf).unwrap();
}

fn conf() -> &'static Conf {
    CONF.get().unwrap()
}

async fn send(phone: String, scene: i8) {
    let access_key_id = "";
    let access_key_secret = "";
    let access_key_id: &str = &access_key_id;
    let access_key_secret: &str = &access_key_secret;

    // RPC接口请求示例一：请求参数"in":"query"   POST
    let method = Method::POST; // 请求方法
    let host = "dysmsapi.aliyuncs.com"; // endpoint
    let canonical_uri = "/"; // RPC接口无资源路径，故使用正斜杠（/）作为CanonicalURI
    let action = "SendSms"; // API名称
    let version = "2017-05-25"; // API版本号

    // 构建 query 参数
    let mut query: Vec<(&str, &str)> = Vec::new();
    let option = conf().templates.get(&scene).ok_or();
    query.push(("PhoneNumbers", option));           // 接收短信手机号
    query.push(("SignName", SMS_SIGN_NAME));              // 短信签名
    query.push(("TemplateCode", TEMPLATE_CODE));         // 模板 Code
    query.push((
        "TemplateParam",
        TEMPLATE_PARAM,                            // JSON 字符串
    ));

    // query 参数
    let query_params: &[(&str, &str)] = &query;

    // 短信接口没有 body
    let body = RequestBody::None;

    // 发起请求
    match call_api(
        method,                                                  // API请求方式 POST/GET/DELETE
        host,                                                    // API服务地址
        canonical_uri,                                           // API资源路径
        query_params,                                            // "in":"query" 查询参数
        action,                                                  // API名称
        version,                                                 // API版本号
        body,                                                    // "in":"body" 请求体参数 支持Json/FormData/Binary类型
        access_key_id,
        access_key_secret,
    )
        .await {
        Ok(response) => println!("响应信息: {}", response),
        Err(error) => eprintln!("异常: {}", error),
    }
}
