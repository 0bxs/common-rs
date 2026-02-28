use reqwest::{Method, StatusCode};
use std::collections::{HashMap};
use std::error::Error;
use std::sync::OnceLock;
use serde::{Deserialize};
use crate::sms::ali_request::{call_api, RequestBody};

#[derive(Debug, Clone)]
pub struct Sms {
    pub access_key: String,
    pub secret_key: String,
    pub sign_name: String,
    pub templates: HashMap<i8, Template>,
}

#[derive(Debug, Clone)]
pub struct Template {
    pub code: String,
    pub param: String,
    pub valid_time: u64,
    pub limit_time: u64,
}

#[derive(Deserialize)]
pub struct AliResVo {
    #[serde(rename = "Code")]
    pub code: String,
    #[serde(rename = "Message")]
    pub message: String,
    #[serde(rename = "BizId")]
    pub biz_id: String,
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

static CONF: OnceLock<Sms> = OnceLock::new();

const HOST: &str = "dysmsapi.aliyuncs.com";
const CANONICAL_URI: &str = "/";
const ACTION: &str = "SendSms";
const VERSION: &str = "2017-05-25";

pub fn init(conf: Sms) {
    CONF.set(conf).unwrap();
}

fn conf() -> &'static Sms {
    CONF.get().unwrap()
}

pub(crate) async fn send(query: Vec<(&str, &str)>) -> Result<(StatusCode, AliResVo), Box<dyn Error>> {
    let res = call_api(Method::POST, HOST, CANONICAL_URI, &query, ACTION,
                       VERSION, RequestBody::None, conf().access_key.as_str(),
                       conf().secret_key.as_str()).await?;
    Ok((res.status(), serde_json::from_slice::<AliResVo>(res.bytes().await?.to_vec().as_slice())?))
}

