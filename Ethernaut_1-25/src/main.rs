mod ethernaut;
mod test;
mod utils;
mod macros;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    ethernaut::level_25::level25_runner()?;

    Ok(())
}
