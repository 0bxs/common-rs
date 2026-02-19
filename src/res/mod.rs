use crate::res::err::ResErr;
use crate::res::ok::ResOk;

pub mod err;
pub mod ok;

pub type Res<T> = Result<ResOk<T>, ResErr>;