use tcm::openssh_runner::SSHRunnerBuilder;
use tcm::shfile::ShFile;

#[tokio::main]
async fn main() -> color_eyre::eyre::Result<()> {
    color_eyre::install()?;
    let s = SSHRunnerBuilder::default()
        .port(12322)
        .user("macwawrz".to_string())
        .host("localhost".to_string())
        .build().await?;
    let f = ShFile::new(s).claim("/etc/hosts").await?;

    println!("{}", f.get_content().await?);

    Ok(())
}
