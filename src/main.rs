use actix_files::Files;
use actix_web::{App, HttpServer};
use dotenv::dotenv;
use file_share::listing_renderer;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    HttpServer::new(|| {
        let data_dir = env::var("DATA_DIR").unwrap_or(String::from("./data"));
        App::new().service(
            Files::new("/", data_dir)
                .show_files_listing()
                .redirect_to_slash_directory()
                .files_listing_renderer(listing_renderer),
        )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
