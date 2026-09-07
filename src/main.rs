use tcm::localrunner::LocalCmdRunner;
use tcm::shfile::ShFile;

#[tokio::main]
async fn main() -> color_eyre::eyre::Result<()> {
    let l = LocalCmdRunner {};
    let f = ShFile::new(l).claim("/etc/hosts").await?;

    println!("{}", f.get_content().await?);

    Ok(())
}
