// 成功
pub const OK: u8 = 1;
// 系统繁忙,请稍后再试
pub const SYSTEM_ERR: u8 = 2;
// accessToken已过期
pub const TOKEN_VALID: u8 = 3;
// refreshToken已过期
pub const REFRESH_VALID: u8 = 4;
// 刷新太早
pub const TOO_EARLY: u8 = 5;
// 未登录
pub const NOT_LOGIN: u8 = 6;
// 没有权限
pub const PERMISSION_DENIED: u8 = 7;
// 自定义异常
pub const CUSTOM_ERR: u8 = 8;
// 请求参数异常
pub const PARAM_ERR: u8 = 9;
