use mongodb::bson::oid::ObjectId;

use crate::core::http::failure::Failure;

pub fn is_valid_object_id(id: &str) -> Result<bool, Failure> {
    let is_valid = ObjectId::parse_str(id).is_ok();
    if is_valid {
        return Ok(true);
    }

    Err(Failure::BadRequest("Id is not valid".to_string()))
}
