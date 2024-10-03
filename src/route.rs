use actix_web::{post, HttpResponse};
use actix_web_actors::ws;
use crate::modle::Web;

#[post("/")]
pub async fn creat_user()->HttpResponse{
    HttpResponse::Ok().body("body")
}


impl actix::Actor for Web{
    type Context = ws::WebsocketContext<Self>;
    fn started(&mut self, ctx: &mut Self::Context) {
        
    }

}