use std::borrow::Cow;
use std::fmt::{Debug, Display, Formatter};
use actix_web::web::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct ResErr {
    pub code: u8,
    pub msg: Cow<'static, str>,
}

impl Debug for ResErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "err:{}", Json(self).0)
    }
}

impl Display for ResErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "err:{}", Json(self).0)
    }
}