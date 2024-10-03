use std::io::Error;

use actix::prelude::*;


// #[derive(Message)]

// #[rtype(result = "Result<bool, std::io::Error>")]
#[derive(Debug)]
struct Q{
    s:String
}
impl Message for Q{
    type Result = Result<bool,std::io::Error>;
}

impl Actor for Q{
    type Context = Context<Self>;
    fn started(&mut self, ctx: &mut Self::Context) {
        println!("started")
    }
    fn stopped(&mut self, ctx: &mut Self::Context) {
        println!("stopped")
    }
    // fn stopping(&mut self, ctx: &mut Self::Context) -> Running {
    //     println!("stopping")
    // }
}
// impl MessageResult<bool,Error> {
    
    
// }
impl Handler<Q> for Q{
    type Result = Result<bool,Error>;
    fn handle(&mut self, msg: Q, ctx: &mut Self::Context) -> Self::Result {
        println!("{:?}",msg);
        Ok(true)
    }
}
impl Supervised for Q{
    fn restarting(&mut self, ctx: &mut <Self as Actor>::Context) {
        println!("{:?}","restarting")
    }
    
}
// impl Actor for Q{
//     type Context = ws::WebsocketContext<Q>;
//     fn started(&mut self, ctx: &mut Self::Context) {
//         println!("qwertyui");
//     }
// }
// impl StreamHandler<> for Q{
    
// }


#[actix::main]
async fn main() {
    let a1=Q{s:"!@#$%%$#@".into()}.start();
    println!("{:?}",a1.send(Q{s:"msg".into()}).await);
    let a2= Q{s:"qwertyui".into()}.start();
    println!("{:?}",size_of_val(&a2));
    println!("Hello, world!{:?}",a1);
}
