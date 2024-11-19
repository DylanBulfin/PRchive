use easifier::{api::{get_issue_events, get_all_issues, get_rate_limit}, error::Result};

fn main() -> Result<()> {
    //let issues = get_issues()?;
    //get_rate_limit()?;
    let events = get_issue_events()?;

    Ok(())
}
