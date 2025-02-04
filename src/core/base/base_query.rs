use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct BaseQuery {
    #[validate(range(min = 1, max = 100))]
    pub page: u32,
    #[validate(range(min = 1, max = 100))]
    pub page_size: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SearchQuery {
    #[serde(flatten)]
    pub base_query: BaseQuery,
    pub search: String,
    #[validate(custom(function = "validate_sort_order"))]
    pub order: String,
}

fn validate_sort_order(order: &str) -> Result<(), ValidationError> {
    let upper = order.to_uppercase();

    return match upper.as_str() {
        "ASC" => Ok(()),
        "DESC" => Ok(()),
        _ => Err(ValidationError::new("Order must be Asc or Desc")),
    };
}
