use crate::diesel;
use diesel::prelude::*;

use actix_web::{web, HttpResponse};
use crate::database::establish_connection;
use crate::schema::to_do_table;

use crate::json_serialization::{to_do_item::ToDoItem, to_do_items::ToDoItems};
use crate::jwt::JwtToken;
use crate::models::item::item::Item;

pub async fn delete(to_do_item: web::Json<ToDoItem>, token: JwtToken) -> HttpResponse {
    let connection = establish_connection();
    let items = to_do_table::table
        .filter(to_do_table::columns::title.eq(&to_do_item.title.as_str()))
        .order(to_do_table::columns::id.asc())
        .load::<Item>(&connection)
        .unwrap();
    let _ = diesel::delete(&items[0]).execute(&connection);

    return HttpResponse::Ok().json(ToDoItems::get_state())
}
