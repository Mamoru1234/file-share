use actix_web::{App, HttpServer};
use actix_files::Files;
use file_share::listing_renderer;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            Files::new("/", ".")
                .show_files_listing()
                .redirect_to_slash_directory()
                .files_listing_renderer(listing_renderer)
        )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
