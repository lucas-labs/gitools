# commit

> Commit files to the repository.
> More information: [https://git-scm.com/docs/git-commit].

# Usage

- Commit staged files to the repository with a message:
    `commit --message "{{message}}"`

- Commit staged files with a message read from a file:
    `commit --file {{path/to/commit_message_file}}`

- Auto stage all modified and deleted files and commit with a message:
    `commit --all --message "{{message}}"`

- Commit staged files and sign them with the specified GPG key (or the one defined in the configuration file if no argument is specified):
    `commit --gpg-sign {{key_id}} --message "{{message}}"`

- Update the last commit by adding the currently staged changes, changing the commit's hash:
    `commit --amend`

- Commit only specific (already staged) files:
    `commit {{path/to/file1 path/to/file2 ...}}`

- Create a commit, even if there are no staged files:
    `commit --message "{{message}}" --allow-empty`