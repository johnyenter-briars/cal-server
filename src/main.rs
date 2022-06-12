use cal_server::db::calconnector::CalConnector;
use cal_server::server::httpserver::build_and_run_server;
use cal_server::{args::programargs::get_args, db::init::initiaize_db};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = get_args();

    let connector = CalConnector::new();

    if args.reset_db {
        let init_result = initiaize_db(args.test_data, &args.user_id, &args.api_key, &connector);
        if init_result.is_err() {
            panic!("Failure to init the DB: {:?}", init_result.err().unwrap());
        }
    }

    build_and_run_server(args.ip, args.port, connector).await
}
