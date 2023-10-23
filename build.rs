use anyhow::{Ok, Result};
use substreams_ethereum::Abigen;

fn main() -> Result<(), anyhow::Error> {
    Abigen::new("tradescrow", "abis/tradescrow.json")?
        .generate()?
        .write_to_file("src/abi/tradescrow.rs")?;

    Ok(())
}
