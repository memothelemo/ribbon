use ribbon_api::InstanceArena;

async fn _start() {
    let arena = InstanceArena::default();
}

#[tokio::main]
async fn main() {
    _start().await;
}
