use serde::Serialize;

#[derive(Serialize)]
pub struct BaseResponse<T> {
    pub data: T,
    pub code: u16,
    pub message: String,
}
