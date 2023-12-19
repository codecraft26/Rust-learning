mod api;

use api::api::task;


use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder,middleware::Logger };
 

#[actix_web::main]

async fn main()-> std::io::Result<()> {
    

    std::env::set_var("RUST_LOG", "actix_web=info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(get_task)
    })
    .bind(("127.0.01",80))?
    .run()
    .await


}
