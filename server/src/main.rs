use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

async fn handle_client(mut tunnel_stream: TcpStream, target_addr: &str) -> io::Result<()> {
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
    // Start the tunnel server
    let tunnel_server_handle = tokio::spawn(async {
        let listener = TcpListener::bind("0.0.0.0:8080")
            .await
            .expect("Tunnel server failed to start");
        println!("Tunnel server running on 0.0.0.0:8080");
        loop {
            let (tunnel_stream, _) = listener
                .accept()
                .await
                .expect("Failed to accept tunnel connection");
            tokio::spawn(handle_client(tunnel_stream, "127.0.0.1:9090"));
        }
    });

    let _ = tokio::select! {
        _ = tunnel_server_handle => {
            eprintln!("Tunnel server failed");
        }
    };

    Ok(())
}
