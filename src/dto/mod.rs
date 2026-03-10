use sea_orm::Order;
use serde::Deserialize;
use validator::Validate;

#[macro_export]
macro_rules! page_dto {
    ($name:ident { $($field:tt)* }) => {
        #[derive(Deserialize, Validate)]
        pub struct $name {
            #[serde(flatten)]
            #[validate(nested)]
            pub page: PageDto,
            $($field)*
        }
    };
}

#[derive(Deserialize, Validate)]
pub struct PageDto {
    #[validate(range(min = 0))]
    pub page_num: u64,
    #[validate(range(min = 1))]
    pub page_size: u64,
    #[validate(length(min = 1))]
    pub order_by: String,
    order_type: OrderType,
}

impl PageDto {
    pub fn order_type(&self) -> Order {
        match &self.order_type {
            OrderType::ASC => Order::Asc,
            OrderType::DESC => Order::Desc,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum OrderType {
    ASC,
    DESC,
}

#[macro_export]
macro_rules! id_page_dto {
    ($name:ident { $($field:tt)* }) => {
        #[derive(Deserialize, Validate)]
        pub struct $name {
            #[serde(flatten)]
            #[validate(nested)]
            pub page: IdPageDto,
            $($field)*
        }
    };
}

#[derive(Deserialize, Validate)]
pub struct IdPageDto {
    #[validate(range(min = 0))]
    pub last_id: i64,
    #[validate(range(min = 1))]
    pub page_size: u64,
}