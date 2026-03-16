use std::sync::OnceLock;
use aliyun_oss_rust_sdk::oss::OSS;
use aliyun_oss_rust_sdk::request::RequestBuilder;
use aliyun_oss_rust_sdk::url::UrlApi;

#[derive(Debug, Clone)]
pub struct Oss {
    pub access_key: String,
    pub secret_key: String,
    pub bucket: String,
    pub endpoint: String,
    pub cdn: bool,
}

static BUCKET: OnceLock<OSS> = OnceLock::new();

static PUT_BUILDER0: OnceLock<RequestBuilder> = OnceLock::new();

static PUT_BUILDER1: OnceLock<RequestBuilder> = OnceLock::new();

static GET_BUILDER: OnceLock<RequestBuilder> = OnceLock::new();

pub fn init(conf: Oss, expire: i64, content_type: &str) {
    let endpoint = conf.endpoint;
    BUCKET.set(OSS::new(conf.access_key, conf.secret_key, endpoint.clone(), conf.bucket)).unwrap();
    let mut builder = RequestBuilder::new().with_expire(expire).
        with_content_type(content_type);
    if conf.cdn {
        builder.cdn = Some(endpoint.clone());
    }
    PUT_BUILDER0.set(builder.clone()).unwrap();
    PUT_BUILDER1.set(builder.oss_header_put("x-oss-acl", "public-read")).unwrap();

    let mut builder = RequestBuilder::new().with_expire(expire);
    if conf.cdn {
        builder.cdn = Some(endpoint);
    }
    GET_BUILDER.set(builder).unwrap();
}

fn bucket() -> &'static OSS {
    BUCKET.get().unwrap()
}

fn put_builder0() -> &'static RequestBuilder {
    PUT_BUILDER0.get().unwrap()
}

fn put_builder1() -> &'static RequestBuilder {
    PUT_BUILDER1.get().unwrap()
}

fn get_builder() -> &'static RequestBuilder {
    GET_BUILDER.get().unwrap()
}

pub fn put_auth_url(file_path: &str, is_public: bool) -> String {
    if is_public {
        bucket().sign_upload_url(file_path, put_builder1())
    } else {
        bucket().sign_upload_url(file_path, put_builder0())
    }
}

pub fn get_auth_url(file_path: &str) -> String {
    bucket().sign_download_url(file_path, get_builder())
}