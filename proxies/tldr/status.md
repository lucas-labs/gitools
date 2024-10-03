# status

> Show the changes to files in a Git repository.
> Lists changed, added and deleted files compared to the currently checked-out commit.
> More information: [https://git-scm.com/docs/git-status].

# Usage

- Show changed files which are not yet added for commit:
    `status`

- Give output in [s]hort format:
    `status --short`

- Show [v]erbose information on changes in both the staging area and working directory:
    `status --verbose --verbose`

- Show the [b]ranch and tracking info:
    `status --branch`

- Show output in [s]hort format along with [b]ranch info:
    `status --short --branch`

- Show the number of entries currently stashed away:
    `status --show-stash`

- Don't show untracked files in the output:
    `status --untracked-files=no`