# checkout

> Checkout a branch or paths to the working tree.
> More information: <https://git-scm.com/docs/git-checkout>.

# Usage

- Create and switch to a new branch:
    `checkout -b {{branch_name}}`

- Create and switch to a new branch based on a specific reference (branch, remote/branch, tag are examples of valid references):
    `checkout -b {{branch_name}} {{reference}}`

- Switch to an existing local branch:
    `checkout {{branch_name}}`

- Switch to the previously checked out branch:
    `checkout -`

- Switch to an existing remote branch:
    `checkout --track {{remote_name}}/{{branch_name}}`

- Discard all unstaged changes in the current directory (see `reset` for more undo-like commands):
    `checkout .`

- Discard unstaged changes to a given file:
    `checkout {{path/to/file}}`

- Replace a file in the current directory with the version of it committed in a given branch:
    `checkout {{branch_name}} -- {{path/to/file}}`