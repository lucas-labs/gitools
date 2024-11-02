# git sync

> This command is a wrapper around:
>   - `git fetch <remote>`
>   - `git rebase <remote>/<branch>`
>   - and optionally `git push <another-remote> <branch>`
> 
> It's intended to be used in a context where you have two or more remotes and you want to sync
> your local copy with one of them without merge commits. The optional push is useful when you
> want to sync your local copy with one remote and push the changes to another remote.
> In summary, this command is useful to keep 3 repositories in sync without merge commits.

# usage

## **case**: get changes from `gitea` into local copy.

`sync from gitea`

**where**:

-   `sync` is the command
-   `from` is a subcommand, just for semantic reasons
-   `gitea` is the name of the remote you want to sync your local copy with

**and will**:

-   `git fetch gitea`
-   `git rebase gitea/master` (assuming you are in the `master` branch)

## **case**: get changes from `gitea` into local copy and push to `github`

`sync from gitea to github`

**where**:

-   `sync` is the command
-   `from` is a subcommand, just for semantic reasons
-   `gitea` is the name of the remote you want to sync your local copy with
-   `to` is a subcommand, just for semantic reasons
-   `github` is the name of the remote you want to push the changes to

**and will**:

-   `git fetch gitea`
-   `git rebase gitea/master` (assuming you are in the `master` branch)
-   `git push github master`

## **case**: get a new branch from `github`

`sync from github:feature-branch`

**where**:

-   `sync` is the command
-   `from` is a subcommand, just for semantic reasons
-   `github:feature/branch` is the remote and branch you want to sync your local copy with

**and will**:

-   `git fetch github feature/branch`
-   `git checkout feature/branch` (this will create a new branch if it doesn't exist)
-   `git rebase github/feature/branch`

## **case**: get a new branch from `github` and push to `gitea`

`sync from github:feature-branch to gitea`

**where**:

-   `sync` is the command
-   `from` is a subcommand, just for semantic reasons
-   `github:feature/branch` is the remote and branch you want to sync your local copy with
-   `to` is a subcommand, just for semantic reasons
-   `gitea` is the name of the remote you want to push the changes to

**and will**:

-   `git fetch github feature/branch`
-   `git checkout feature/branch` (this will create a new branch if it doesn't exist)
-   `git rebase github/feature/branch`
-   `git push gitea feature/branch`

# Note:

Before running all the commands, this program will show you the commands that will be executed
and ask for your confirmation. If you don't confirm, the program will exit without running any
command.
