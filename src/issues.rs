use octocrab::Octocrab;

use crate::{error::Result, state::IssuePair};

/// This module contains the code related to fetching and parsing Issue/PR pairs for a
/// repository.

const LABEL: &str = "good-first-issue";

pub async fn get_issue_pairs(owner: &str, repo: &str) -> Result<Vec<IssuePair>> {
    let crab = Octocrab::builder()
        .base_uri("https://github.com")?
        .build()?;

    let labels = vec![LABEL.to_string()];

    let issues = crab
        .issues(owner, repo)
        .list()
        .labels(&labels)
        .send()
        .await?;

    unimplemented!()
}
