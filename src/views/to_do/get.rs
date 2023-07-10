use actix_web::{web, Responder};
use serde_json::value::Value;
use serde_json::Map;
use crate::jwt::JwtToken;

use crate::to_do::{ItemTypes, to_do_factory, enums::TaskStatus};
use crate::json_serialization::to_do_items::ToDoItems;

pub async fn get(token: JwtToken) -> impl Responder {
    return ToDoItems::get_state(token.user_id);
}
