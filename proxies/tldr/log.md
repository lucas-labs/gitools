# log

> Show a history of commits.
> More information: [https://git-scm.com/docs/git-log].

# Usage

- Show the sequence of commits starting from the current one, in reverse chronological order of the Git repository in the current working directory:
    `log`

- Show the history of a particular file or directory, including differences:
    `log {{--patch|-p|-u}} {{path/to/file_or_directory}}`

- Show an overview of which file(s) changed in each commit:
    `log --stat`

- Show a graph of commits in the current branch using only the first line of each commit message:
    `log --oneline --graph`

- Show a graph of all commits, tags and branches in the entire repo:
    `log --oneline --decorate --all --graph`

- Show only commits with messages that include a specific string, ignoring case:
    `log {{-i|--regexp-ignore-case}} --grep {{search_string}}`

- Show the last N number of commits from a certain author:
    `log {{--max-count|-n} {{number}} --author "{{author}}"`

- Show commits between two dates (yyyy-mm-dd):
    `log --before "{{2017-01-29}}" --after "{{2017-01-17}}"`