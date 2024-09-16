# pull

> Fetch branch from a remote repository and merge it to local repository.
> More information: <https://git-scm.com/docs/git-pull>.

# Usage

- Download changes from default remote repository and merge it:
    `pull`

- Download changes from default remote repository and use fast-forward:
    `pull --rebase`

- Download changes from given remote repository and branch, then merge them into HEAD:
    `pull {{remote_name}} {{branch}}`