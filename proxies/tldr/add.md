# add

> Adds changed files to the index.
> More information: <https://git-scm.com/docs/git-add>.

# Usage

- Add a file to the index:
    `add {{path/to/file}}`

- Add all files (tracked and untracked):
    `add {{-A|--all}}`

- Add all files in the current folder:
    `add .`

- Only add already tracked files:
    `add {{-u|--update}}`

- Also add ignored files:
    `add {{-f|--force}}`

- Interactively stage parts of files:
    `add {{-p|--patch}}`

- Interactively stage parts of a given file:
    `add {{-p|--patch}} {{path/to/file}}`

- Interactively stage a file:
    `add {{-i|--interactive}}`