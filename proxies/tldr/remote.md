# remote

> Manage set of tracked repositories ("remotes").
> More information: [https://git-scm.com/docs/git-remote].

# Extension

This command is an extension of the original `git remote`
command that adds a `list` subcommand to list all remotes
in a more human-readable format.

`remote list` is equivalent to `git remote -v`.

# Usage

- List existing remotes with their names and URLs:
    `remote list` (extended: `remote -v`),
    or `git remote {{-v|--verbose}}`

- Show information about a remote:
    `remote show {{remote_name}}`

- Add a remote:
    `remote add {{remote_name}} {{remote_url}}`

- Change the URL of a remote (use `--add` to keep the existing URL):
    `remote set-url {{remote_name}} {{new_url}}`

- Show the URL of a remote:
    `remote get-url {{remote_name}}`

- Remove a remote:
    `remote remove {{remote_name}}`

- Rename a remote:
    `remote rename {{old_name}} {{new_name}}`