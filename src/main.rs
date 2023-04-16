#[actix_rt::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug, actix_Server=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
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
