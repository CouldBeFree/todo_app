use crate::diesel;
use diesel::prelude::*;
use actix_web::{web, HttpResponse};

use crate::json_serialization::{to_do_item::ToDoItem, to_do_items::ToDoItems};
use crate::jwt::JwtToken;
use crate::database::establish_connection;
use crate::schema::to_do_table;
use crate::database::DB;

pub async fn edit(to_do_item: web::Json<ToDoItem>, token: JwtToken, db: DB) -> HttpResponse {
    let results = to_do_table::table
        .filter(to_do_table::columns::title.eq(&to_do_item.title));
    
    let _ = diesel::update(results)
        .set(to_do_table::columns::status.eq("DONE"))
        .execute(&db.connection);
    
    return HttpResponse::Ok().json(ToDoItems::get_state())
}
