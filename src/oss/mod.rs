use std::sync::OnceLock;
use aliyun_oss_rust_sdk::oss::OSS;
use aliyun_oss_rust_sdk::request::RequestBuilder;
use aliyun_oss_rust_sdk::url::UrlApi;

#[derive(Debug, Clone)]
pub struct Conf {
    pub access_key: String,
    pub secret_key: String,
    pub bucket: String,
    pub endpoint: String,
    pub cdn: bool,
}

static BUCKET: OnceLock<OSS> = OnceLock::new();

static BUILDER: OnceLock<RequestBuilder> = OnceLock::new();

pub fn init(conf: Conf) {
    let endpoint = conf.endpoint;
    BUCKET.set(OSS::new(conf.access_key, conf.secret_key, endpoint.clone(), conf.bucket)).unwrap();
    let mut builder = RequestBuilder::new().with_expire(60);
    if conf.cdn {
        builder.cdn = Some(endpoint);
    }
    BUILDER.set(builder).unwrap();
}

fn bucket() -> &'static OSS {
    BUCKET.get().unwrap()
}

fn builder() -> &'static RequestBuilder {
    BUILDER.get().unwrap()
}

pub fn put_auth_url(file_path: &str) -> String {
    bucket().sign_download_url(file_path, builder())
}

pub fn get_auth_url(file_path: &str) -> String {
    bucket().sign_upload_url(file_path, builder())
}