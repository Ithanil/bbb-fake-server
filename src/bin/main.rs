use loco_rs::cli;
use bbb_fake_server::app::App;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    cli::main::<App>().await
}
