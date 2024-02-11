use crate::auth::{login_user, register_user};
use actix_files as fs;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .route("/register", web::post().to(register_user))
            .route("/login", web::post().to(login_user)),
    );
    cfg.service(fs::Files::new("/", "./build").show_files_listing());
    cfg.default_service(web::route().to(index));
}

async fn index() -> actix_web::Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("./build/index.html")?)
}
