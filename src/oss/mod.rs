use aliyun_oss_rust_sdk::oss::OSS;
use aliyun_oss_rust_sdk::request::RequestBuilder;
use aliyun_oss_rust_sdk::url::UrlApi;
use std::sync::OnceLock;

#[derive(Debug, Clone)]
pub struct Oss {
    pub access_key: String,
    pub secret_key: String,
    pub bucket: String,
    pub endpoint: String,
    pub cdn: bool,
}

static BUCKET: OnceLock<OSS> = OnceLock::new();

static PUT_BUILDER: OnceLock<RequestBuilder> = OnceLock::new();

static GET_BUILDER: OnceLock<RequestBuilder> = OnceLock::new();

pub fn init(conf: Oss, expire: i64, content_type: &str) {
    let endpoint = conf.endpoint;
    BUCKET
        .set(OSS::new(
            conf.access_key,
            conf.secret_key,
            endpoint.clone(),
            conf.bucket,
        ))
        .unwrap();
    let mut builder = RequestBuilder::new()
        .with_expire(expire)
        .with_content_type(content_type);
    if conf.cdn {
        builder.cdn = Some(endpoint.clone());
    }
    PUT_BUILDER.set(builder).unwrap();

    let mut builder = RequestBuilder::new().with_expire(expire);
    if conf.cdn {
        builder.cdn = Some(endpoint);
    }
    GET_BUILDER.set(builder).unwrap();
}

fn bucket() -> &'static OSS {
    BUCKET.get().unwrap()
}

fn put_builder() -> &'static RequestBuilder {
    PUT_BUILDER.get().unwrap()
}

fn get_builder() -> &'static RequestBuilder {
    GET_BUILDER.get().unwrap()
}

pub fn put_auth_url(file_path: &str) -> String {
    bucket().sign_upload_url(file_path, put_builder())
}

pub fn get_auth_url(file_path: &str) -> String {
    bucket().sign_download_url(file_path, get_builder())
}
