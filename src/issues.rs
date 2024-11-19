use futures::executor::{self, block_on};
use octocrab::{models::issues, params::State, service::middleware::extra_headers::ExtraHeaders, Octocrab, Page};

use crate::{error::Result, state::IssuePair};

/// This module contains the code related to fetching and parsing Issue/PR pairs for a
/// repository.

const LABEL: &str = "good-first-issue";

pub async fn get_issues_page(
    owner: &str,
    repo: &str,
    crab: &Octocrab,
    labels: &[String],
    page: u32,
) -> Result<Page<issues::Issue>> {

}

pub async fn get_all_issues(
    owner: String,
    repo: String,
    crab: &Octocrab,
    labels: &[String],
) -> Result<Vec<issues::Issue>> {
    
}

pub fn get_issue_pairs(owner: String, repo: String) -> Result<Vec<IssuePair>> {
    
}
