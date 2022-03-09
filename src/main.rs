use actix_web::{get, web, App, HttpServer, Responder};
use cal_server::args::programargs::get_args;
use cal_server::db::init::initiaize_db;
use cal_server::routes::event::create_event;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = get_args();

    println!("{:?}", args);

    if args.reset_db {
        let init_result = initiaize_db();
        if init_result.is_err() {
            panic!("Failure to init the DB!: {:?}", init_result.err().unwrap());
        }
    }

    HttpServer::new(|| App::new().service(create_event))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await

    // HttpServer::new(|| {
    //     App::new()
    //         .route("/hello")
    //         .service(create_event)
    // })
    // .bind(("127.0.0.1", 8080))?
    // .run()
    // .await
}
