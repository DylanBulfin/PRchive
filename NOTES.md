# Notes
These are just my thoughts about ongoing efforts in this project

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


