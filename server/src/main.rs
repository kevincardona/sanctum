mod jwt;
mod auth;
mod db;
mod models;
mod routes;

use tokio::io;
use dotenv::dotenv;
use actix_web::{App, web::Data, HttpServer};
use tokio::net::{TcpListener, TcpStream};

async fn handle_tunnel_client(mut tunnel_stream: TcpStream, target_addr: &str) -> io::Result<()> {
    let mut target_stream = TcpStream::connect(target_addr).await?;
    let (mut tunnel_reader, mut tunnel_writer) = tunnel_stream.split();
    let (mut target_reader, mut target_writer) = target_stream.split();

    let client_to_target = io::copy(&mut tunnel_reader, &mut target_writer);
    let target_to_client = io::copy(&mut target_reader, &mut tunnel_writer);

    tokio::try_join!(client_to_target, target_to_client)?;
    Ok(())
}

#[tokio::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
   
    // Web Server
    let pool = db::setup_database().await;
    let actix_server = HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .configure(routes::config)
    })
    .bind("127.0.0.1:9090")?
    .run();

    let actix_handle = tokio::spawn(async move {
        match actix_server.await {
            Ok(_) => println!("Actix-web server running successfully."),
            Err(e) => eprintln!("Actix-web server failed: {}", e),
        }
    });
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    // Tunnel Server
    let tunnel_server_handle = tokio::spawn(async {
        let listener = TcpListener::bind("0.0.0.0:8081")
            .await
            .expect("Tunnel server failed to start");
        println!("Tunnel server running on 0.0.0.0:8081");
        loop {
            let (tunnel_stream, _) = listener
                .accept()
                .await
                .expect("Failed to accept tunnel connection");
            tokio::spawn(handle_tunnel_client(tunnel_stream, "127.0.0.1:9090"));
        }
    });

    let _ = tokio::select! {
        _ = tunnel_server_handle => {
            eprintln!("Tunnel server failed");
        }
    };

    // wait for the actix-web server to finish
    let _ = actix_handle.await;

    Ok(())
}
