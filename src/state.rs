/// This module contains code related to the runtime state of the easifier app. This
/// includes:
/// - A list of repositories stored locally, along with the Issue/PR pair the user has
/// checked out

pub enum ActorType {
    Owner,
    Admin,
    Maintainer,
    Collaborator,
    Contributor,
    None,
}

/// This contains information about the actor associated with certain actions. Mainly
/// useful to distinguish users as contributors/maintainers.
pub struct Actor {
    username: String,
    profile_url: String,
    actor_type: ActorType,
}

pub struct Comment {
    author: Actor,
    content: String,
}

pub struct Issue {
    author: Actor,
    issue_number: u64,
}

pub struct PullRequest {
    author: Actor,
    issue_number: u64,

    // The hash of the merge commit
    merge_commit_hash: String,
    // The hash of the ref the PR code was based on
    base_commit_hash: String,
    // Full list of constituent commits for the PR
    commits: Vec<String>,
}

pub struct IssuePair {
    issue: Issue,
    pull_request: PullRequest,
}

pub struct Repository {
    remote_url: String,
    cached_pairs: Vec<IssuePair>,
}
