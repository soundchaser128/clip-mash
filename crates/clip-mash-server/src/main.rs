use clip_mash::Result;

#[tokio::main]
async fn main() -> Result<()> {
    clip_mash_server::start_server().await
}
