# Notes
These are just my thoughts about ongoing efforts in this project

## Sorting
It's an open question how I should sort issues.

### Descending (newest -> oldest)
Pros:
- Most closely represents the current state of the codebase which makes it potentially the
  best way to learn

Cons:
- Many popular projects have been more or less complete for years so newer issues are 
  likely to be less interesting/informative (think documentation typos)
- Confusing to have the codebase get smaller and less mature as you work through issues

### Ascending (oldest -> newest)
Pros:
- Potentially better-quality beginner issues
- Easier onboarding since early issues will involve a smaller, easier to navigate codebase

Cons:
- When these projects were not mature they might not have had a lot of outside 
  contributors, meaning they may not have utilized the `good-first-issue` label, and there
  might not be as many useful discussions
- Some projects have gone through gigantic rewrites which would be hard to detect
  programatically. This would make onboarding confusing
- While Rust hasn't changed that dramatically since the old days, it has changed. It may 
  be confusing if the user creates a solution that uses newer library items and has to
  wonder why the accepted solution was so different 
    - e.g. creating a static variable with non-const functions would have required 
      `lazy_static` before but now would likely use the newly-stabilized 
      `LazyCell/LazyLock`)

### Alternatives
1. Fetch and cache all closed Issues/PRs when initializing a repository. Then scrub
   through, identify any larger-than-normal diffs, designate those as potential rewrites.
   Give the user all valid issues newer than this, sorted ascending
    - This means initializing a large repository would use up a ton of API requests. At
      100/page max a large repo like `nixpkgs` would take ~3500 requests, or around 2/3 of
      the hourly rate limit for authenticated users
    - Identifying large changes is difficult. We'd want to identify PRs that touch over
      a certain percentage of the code for a certain percentage of files, but those
      numbers will be hard to decide on
2. Fetch the earliest good-first-issue pairs as normal, and compare the file state in
   those commits to the current state. If a certain percentage of the files from that
   commit are still around we can assume that the structure hasn't changed a ton
    - If they are too different we can use binary search to find a state that is valid
    - Probably worth fetching the state of the code at multiple points anyway, since there
      could have been a rewrite relatively recently that happened to not touch the core
      code

The limitation of the above approaches is that many projects went through module
reorganizaiton. E.g. in the 
[first `good-first-issue`-associated PR for rust-clippy](https://github.com/rust-lang/rust-clippy/pull/300), 
the lint was added to `src/minmax.rs`. It still exists but is now at 
`clippy_lints/src/minmax.rs`. Maybe we should check recursively each folder in the current 
code state, to see if this kind of reorganization happened (we would warn the user in some 
way)

In addition to the above changes I would want to ensure they were written with a vagely
recent version of Rust. Compare the version of the above lint from the original PR 
[here](https://github.com/rust-lang/rust-clippy/blob/79bf820170faf257e68540d43cdf40112822a87d/src/minmax.rs) with the newest version
[here](https://github.com/rust-lang/rust-clippy/blob/master/clippy_lints/src/minmax.rs).
They almost seem like different languages; the original is similar to something I might
write today, whereas the newest is the kind of code most Rust projects use, and the latter
is clearly much more important to learn. I can't say for sure how much of that is
developer maturity and how much is language features changing, but I think it suggests we
should consider limiting PRs to either the last 5 years or so, or to 1-2 years after the
project began, or both

## Pull Request/Issues
As of now I'm pretty sure we can check the timeline for events 


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
