# PRchive
It's pronounced like 'p archive'.

Some notes are in NOTES.md as well.

PRchive is a tool I've designed to help people (such as myself) get experience in open 
source. A problem I've had with open source in the past is how difficult it can be to get
into. I find it hard to learn about a new project solely by reading the code, even if it's
heavily commented (which many aren't). It's hard to figure out what parts are worth 
focusing on and it's generally just very tedious. I think the best way to learn is by 
doing. But most open source projects I run into that seem interesting fall into one of 
the following categories:
1. The codebase is relatively small and easy to learn, but there are not many issues
   overall and even fewer currently open
2. The codebase is manageable but actively developed. There are a lot of issues to work on
   but they get claimed and fixed very quickly
3. The codebase is extremely large/complex, and the only issues I would be capable of
   solving with my level of experience are things like fixing typos, which aren't really
   educational. These also tend to be the most popular projects so any simple issues that
   do come up are taken immediately

The idea of PRchive is that a project in any one of these categories likely has a rich
history of development, with plenty of interesting problems solved, accompanied by
countless educational discussions. We can harness that to learn the codebase in a more
convenient way, by working on the best beginner issues from the history of the repository.

## Usage
Nothing is implemented yet but my idea for how it will work is as follows:
1. User uses PRchive to browse a list of closed issues with a `good-first-issue` label,
   seeing the issue title and body, and selects one
2. PRchive clones the repository if it isn't already, and checks out a commit right before
   the issue's corresponding PR was merged
3. The user goes into the code and fixes the issue, viewing comments from the issue or the
   PR if they get stuck
4. The program will show the user the solution that ended up being merged, along with a
   diff with their solution

The benefit of this approach is two-fold:
1. Allows the user a wider selection of simple issues to work on in order to learn
2. Gives the user a full solution to compare their results to immediately, rather than
   waiting on PR reviews

## Language Compatibility
As of right now since we only clone the repository this is not necessary, but eventually I
would like to incorporate some basic testing support for rust specifically. I may
eventually try extending support to other languages that have backward compatibility
(needed because I won't be handling different compiler versions or anything like that)

## Goals
Since this is built entirely on others' open source contributions, consistently giving
proper attribution (linking people's profiles and usernames, etc) is very important.
Mostly leaving this here as a reminder to myself
