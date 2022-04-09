use cal_server::{
    args::programargs::get_args, db::init::initiaize_db, server::build_and_run_server,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = get_args();

    if args.reset_db {
        let init_result = initiaize_db(args.test_data);
        if init_result.is_err() {
            panic!("Failure to init the DB: {:?}", init_result.err().unwrap());
        }
    }

    build_and_run_server(args.ip, args.port, args.api_key).await
}
