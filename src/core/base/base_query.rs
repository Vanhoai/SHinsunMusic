use serde::{Deserialize, Serialize};
use std::str::FromStr;
use validator::{Validate, ValidationError};

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct BaseQuery {
    #[serde(deserialize_with = "parse_usize")]
    #[validate(range(min = 1, max = 100))]
    pub page: usize,
    #[serde(deserialize_with = "parse_usize")]
    #[validate(range(min = 1, max = 100))]
    pub page_size: usize,
}

fn parse_usize<'de, D>(deserializer: D) -> Result<usize, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    usize::from_str(s).map_err(serde::de::Error::custom)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchQuery {
    #[serde(flatten)]
    pub base_query: BaseQuery,
    pub search: String,
}

pub fn validate_sort_order(order: &str) -> Result<(), ValidationError> {
    let upper = order.to_uppercase();

    match upper.as_str() {
        "ASC" => Ok(()),
        "DESC" => Ok(()),
        _ => Err(ValidationError::new("Order must be Asc or Desc")),
    }
}
