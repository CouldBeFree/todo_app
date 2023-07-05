use crate::schema::to_do_table;
use chrono::NaiveDateTime;

#[derive(Queryable, Identifiable)]
#[table_name="to_do_table"]
pub struct Item {
    pub id: i32,
    pub title: String,
    pub status: String,
    pub date: NaiveDateTime
}
