use actix_web::Responder;
use crate::jwt::JwtToken;

use crate::json_serialization::to_do_items::ToDoItems;

pub async fn get(token: JwtToken) -> impl Responder {
    return ToDoItems::get_state(token.user_id);
}
