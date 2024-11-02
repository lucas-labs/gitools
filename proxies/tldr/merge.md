# merge

> Merge branches.
> More information: [https://git-scm.com/docs/git-merge].

# Usage

- Merge a branch into your current branch:
    `merge {{branch_name}}`

- Edit the merge message:
    `merge --edit {{branch_name}}`

- Merge a branch and create a merge commit:
    `merge --no-ff {{branch_name}}`

- Abort a merge in case of conflicts:
    `merge --abort`

- Merge using a specific strategy:
    `merge --strategy {{strategy}} --strategy-option {{strategy_option}} {{branch_name}}`