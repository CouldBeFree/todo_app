use crate::diesel;
use diesel::prelude::*;

use actix_web::HttpResponse;
use actix_web::HttpRequest;

use crate::json_serialization::to_do_items::ToDoItems;

use crate::database::establish_connection;
use crate::models::item::new_item::NewItem;
use crate::models::item::item::Item;
use crate::schema::to_do_table;
use crate::jwt::JwtToken;
use crate::database::DB;

pub async fn create(token: JwtToken, req: HttpRequest, db: DB) -> HttpResponse {
    let title: String = req.match_info().get("title").unwrap().to_string();
    let connection = establish_connection();
    let items = to_do_table::table
        .filter(to_do_table::columns::title.eq(&title.as_str()))
        .order(to_do_table::columns::id.asc())
        .load::<Item>(&connection)
        .unwrap();

    if items.len() == 0 {
        let new_post = NewItem::new(title, token.user_id);
        let _ = diesel::insert_into(to_do_table::table).values(&new_post)
            .execute(&connection);
    }

    return HttpResponse::Ok().json(ToDoItems::get_state(token.user_id));
}