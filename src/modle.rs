use diesel::{Queryable,Insertable,Selectable,};
use chrono::{NaiveDate, NaiveDateTime};
use crate::schema::*;
use serde::{Serialize,Deserialize};

#[derive(Debug,Queryable,Insertable,Selectable,Serialize,Deserialize)]
#[diesel(table_name = users)]
pub struct Users{
    pub user_id:Option<i32>,
    pub username:String,
    pub email_id:String,
    pub dob:NaiveDate
}

#[derive(Debug,Queryable,Insertable,Selectable,Serialize,Deserialize)]
#[diesel(table_name = auth)]
pub struct Auth{
    pub user_id:i32,
    pub passwd:String
}

#[derive(Debug,Queryable,Insertable,Selectable,Serialize,Deserialize)]
#[diesel(table_name = chats)]
pub struct Chats{
    pub id:Option<i64>,
    pub from_:i32,
    pub to_:i32,
    pub msg:String,
    pub sent:NaiveDateTime,
    pub received:Option<NaiveDateTime>
}

#[derive(Debug,Deserialize)]
pub struct ChatText{
    pub msg:String
}

#[derive(Debug,Serialize,Deserialize)]
pub struct ChatResponse{
    pub id:i64,
    pub ack:i64,
}

#[derive(Debug,Serialize,Deserialize)]
pub struct ChatS2C{
    pub from:i64,
    pub id:i64,
    pub msg:String,
    pub sent:i64
}

#[derive(Debug,Serialize,Deserialize)]
pub struct Web{}