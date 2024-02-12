use crate::auth::{login_user, register_user, is_authenticated};
use actix_files as fs;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .service(register_user)
            .service(login_user)
            .service(is_authenticated),
    );
    cfg.service(fs::Files::new("/", "./build").show_files_listing());
    cfg.default_service(web::route().to(index));
}

async fn index() -> actix_web::Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("./build/index.html")?)
}
