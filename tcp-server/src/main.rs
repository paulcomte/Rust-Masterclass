#[tokio::main]
pub async fn main() {
    let target_host = "0.0.0.0";
    let port = 3000;

    let (_server_manager, handle) = network::run_server(target_host, port).await;

    eprintln!("Exit result: {:#?}", handle.await);
}
