use clap::Parser;
use rgrep::GrepConfig;
use anyhow::Result;

fn main() -> Result<()> {
    let config: GrepConfig = GrepConfig::parse();
    config.match_with_default_strategy()?;
    Ok(())
}