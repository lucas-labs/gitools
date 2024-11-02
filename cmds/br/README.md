# br

> This command is a git utility to manage branches.
> Note: This command is not exactly the same as the `git branch` command 
> or `branch` gitools proxy command.

# usage

- Display a select menu to checkout a branch:
    `br`

- List all branches (local and remote; the current branch is highlighted by `*`):
    `br --list`

- Checkout a branch by name:
    `br {{branch_name}}`

- Create new branch based on the current commit:
    `br {{branch_name}}`

- Remove a branch:
    `br -d {{branch_name}}`
