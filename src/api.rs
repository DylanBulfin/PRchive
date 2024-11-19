use crate::error::{Error, Result};
use serde::{Deserialize, Deserializer};
use ureq::{serde_json::Value, Request};

macro_rules! template {
    (RateLimit) => {
        "https://api.github.com/rate_limit"
    };
    (GetIssues) => {
        "https://api.github.com/repos/{}/{}/issues?labels=good-first-issue&state=closed"
    };
    (GetTimelineEvents) => {
        "https://api.github.com/repos/{}/{}/issues/{}/timeline"
    };
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct User {
    /// The username of the actor
    login: String,
    /// The url of the actor's page
    html_url: String,
    //actor_type: ActorType,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ActorType {
    Owner,
    Admin,
    Maintainer,
    Collaborator,
    Contributor,
    None,
}

/// This contains information about the actor associated with certain actions
#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct EventActor {
    /// The username of the actor
    login: String,
    /// The url of the actor's page
    html_url: String,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum EventType {
    CrossReferenced,
    // Referenced,
    // Mentioned,
    // Connected,
    #[serde(other)]
    Other,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct CrossReferenceSource {
    #[serde(rename = "type")]
    ty: String,
    issue: IssueData,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Event {
    actor: EventActor,
    // This field is present for cross-reference type events, which are the main way PRs
    // seem to be tied to issues.
    source: Option<CrossReferenceSource>,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum State {
    Open,
    Closed,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum StateReason {
    Completed,
    Reopened,
    NotPlanned,
}

#[derive(Deserialize, Debug)]
pub struct IssuePR {}

#[derive(Deserialize, Debug)]
pub struct IssueData {
    number: u64,
    state: State,
    state_reason: Option<StateReason>,
    title: String,
    body: Option<String>,
    user: Option<User>,
    pull_request: Option<IssuePR>,
    // AuthorAssociation should be supported, it's in the API
}


pub trait AddHeaders: Sized {
    fn add_default_headers(self) -> Self;
}

impl AddHeaders for Request {
    fn add_default_headers(self) -> Self {
        self.set("X-GitHub-Api-Version", "2022-11-28")
            .set("Accept", "application/vnd.github+json")
    }
}

pub fn get_rate_limit() -> Result<()> {
    let rate_limit = ureq::get(template!(RateLimit))
        .add_default_headers()
        .call()?
        .into_string()?;

    panic!("{:?}", rate_limit);
}

pub fn get_filtered_issues() -> Result<Vec<IssueData>> {
    unimplemented!()
}

pub fn get_all_issues() -> Result<Vec<IssueData>> {
    let url = format!(template!(GetIssues), "rust-lang", "rust-clippy");

    let all_issues = ureq::get(&url)
        .add_default_headers()
        .call()?
        .into_json::<Vec<IssueData>>()?;

    Ok(all_issues
        .into_iter()
        .filter(|i| i.pull_request.is_none() && i.state_reason == Some(StateReason::Completed))
        .collect())
}

pub fn get_issue_events(number: u64) -> Result<Vec<Event>> {
    let url = format!(
        template!(GetTimelineEvents),
        "rust-lang", "rust-clippy", number
    );

    let all_events = ureq::get(&url)
        .add_default_headers()
        .call()?
        .into_json::<Vec<Event>>()?;

    panic!("{:?}", all_events);

    //Ok(all_events
    //    .into_iter()
    //    .filter(|i| i.pull_request.is_none() && i.state_reason == Some(StateReason::Completed))
    //    .collect())
}
