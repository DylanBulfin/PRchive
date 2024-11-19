# Notes
These are just my thoughts about ongoing efforts in this project

## Async
Octocrab does a lot of things async by default. I don't think an application like this
is likely to benefit from async execution since most of the work is short running and done
directly at the user's direction. So for now I use `futures::block_on`. This may change in
the future as I learn more about async rust. 

## Pull Request/Issues
- The issues `GET` endpoint also returns PRs but PRs have additional field with links to commit and such
- Issue mentions are tracked as 'events' which we can get for either

So good issues for us fulfill the following criteria:
- The issue is mentioned in exactly one PR
- The PR and issue are both closed
- The PR was merged

The first requirement is so that we don't have to deal with ambiguity. If there are 2 PRs that mention an issue, how do
we know if one or both of them solve the issue?

Additionally we probably want to limit ourselves to issue-PR pairs where the PR was merged and the issue closed around
the same time (within a day, say, or maybe longer or shorter depending on how active the project is)

## Things to find out
- Do PRs tell you the branch they were merged into?
    - Since we want to checkout the commit immediately before the PR merge we need more information than just the SHA



## GraphQL
GitHub has a GraphQL API which might allow simplified fetching code. I've identified the
following query that fetches any PRs that have referenced an issue, and gets their PR
number and the commit HASH the PR branch is based on
```
query {
  repository(owner: "rust-lang", name: "rust-clippy") {
    issue(number: 12542) {
      timelineItems(first: 100) {
        nodes {
          ... on CrossReferencedEvent {
           source {
              ... on PullRequest {
                number
                baseRefOid
              }
            }
          }
        }
	  }
    }
  }
}
```
