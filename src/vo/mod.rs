use serde::Serialize;

#[derive(Serialize)]
pub struct PageVo<T> {
    pub element: Vec<T>,
    pub total_elements: u64,
}

impl<T> PageVo<T> {
    pub fn of(element: Vec<T>, count: u64) -> PageVo<T> {
        PageVo {
            element,
            total_elements: count,
        }
    }
}
