/// This module contains code related to the runtime state of the easifier app. This
/// includes:
/// - A list of repositories stored locally, along with the Issue/PR pair the user has
/// checked out

//pub struct Comment {
//    author: Actor,
//    content: String,
//}

use crate::api::IssueData;

pub struct Issue {

}

pub struct PullRequest {
    //author: Actor,
    issue_number: u64,

    // The hash of the merge commit
    merge_commit_hash: String,
    // The hash of the ref the PR code was based on
    base_commit_hash: String,
    // Full list of constituent commits for the PR
    commits: Vec<String>,
}

pub struct IssuePair {
    issue: IssueData,
    pull_request: PullRequest,
}

pub struct Repository {
    remote_url: String,
    cached_pairs: Vec<IssuePair>,
}
