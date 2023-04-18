#[actix_rt::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug, actix_Server=info");
    env_logger::init();

    // -- setting up database connection
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool");

    HttpServer::new(|| {
        App::new()
            // -- Set up db pool for use with web::Data::<Pool> extractor
            .data(pool.clone())
            // -- enable logger always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
            // -- register http requests handlers
            .service(peep::list)
            .service(peep::get)
            .service(peep::create)
            .service(peep::delete)
            .service(like::list)
            .service(like::plus_one)
            .service(like::minus_one)
    })
    .bind("0.0.0.0:9090")?
    .run()
    .await
}
