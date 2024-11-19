use easifier::{error::Result, issues::get_issue_pairs};

#[tokio::main]
async fn main() -> Result<()> {
    get_issue_pairs("rust-lang".to_string(), "rust-clippy".to_string())?;

    Ok(())
}
