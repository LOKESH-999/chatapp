use chrono::{NaiveDateTime, Utc};
use diesel::query_dsl::methods::{FilterDsl, SelectDsl};
use diesel:: r2d2::{self, ConnectionManager};
use diesel::result::Error;
use diesel::{ExpressionMethods, SelectableHelper};
use crate::modle::*;
use crate::schema::*;
use diesel::RunQueryDsl;

pub type DBPOOL=r2d2::Pool<ConnectionManager<diesel::pg::PgConnection>>;


pub fn creat_user(conn:&DBPOOL,user:Users)->Result<Users,Error>{
    diesel::insert_into(users::dsl::users)
    .values(user).returning(Users::as_returning())
    .get_result(&mut *conn.get().unwrap())
}

pub fn add_chat(conn:&DBPOOL,from_:i32,to_:i32,msg:String)->Result<Chats,Error>{
    let ch=Chats{
        id:None,
        from_,
        msg,
        to_,
        sent:Utc::now().naive_utc().into(),
        received:None
    };
    diesel::insert_into(chats::dsl::chats)
    .values(ch)
    .returning(Chats::as_returning())
    .get_result(&mut *conn.get().unwrap())
}

//need to remove output or return statement
pub fn update_chat(conn:&DBPOOL,id:i64,time:NaiveDateTime)->Result<Chats,Error>{
    diesel::update(chats::dsl::chats)
    .filter(chats::dsl::id.eq(id))
    .set(chats::received.eq(time))
    .returning(Chats::as_returning())
    .get_result(&mut *conn.get().unwrap())
}

pub fn add_auth(conn:&DBPOOL,id:i32,passwd:String)->Result<Auth,Error>{
    diesel::insert_into(auth::dsl::auth)
    .values(Auth{
        user_id:id,
        passwd
    }).returning(Auth::as_returning())
    .get_result(&mut *conn.get().unwrap())
}


pub fn get_user_by_id(conn:&DBPOOL,id:i32)->Result<Users,Error>{
    users::dsl::users.select((users::user_id,users::username,users::email_id,users::dob))
    .filter(users::user_id.eq(Some(id)))
    .get_result::<Users>(&mut *conn.get().unwrap())
}

// pub fn 
