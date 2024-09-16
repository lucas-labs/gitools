# Gitools

Gitools is a collection of utilities that simplify working with Git and GitHub. The goal is to
streamline common tasks by reducing the amount of typing and adding helpful tools to improve
productivity.

## Features

- **Command Shortcuts**: Run frequently used Git commands without having to prepend `git` every
    time. For example:

  - `checkout`, `co`, `chkout` instead of `git checkout`
  - `status`, `stat` instead of `git status`
  - `commit`, `c`, `cmt` instead of `git commit`
  - And more for other common commands!

- **Additional Tools**: Gitools will also include various utilities to make working with Git and
  GitHub easier. More tools will be added over time to enhance and streamline your workflows.

# Packages

## git-tools » proxies

### Overview

The `proxies` package is part of the `git-tools` workspace, providing a collection of proxy commands
for interacting with Git. This package includes several binaries that proxify Git commands,
providing shortcuts.

### Binaries

The `proxies` package includes the following binaries:

- `status`, `st`: Proxifies the `git status` command.
- `add`: Proxifies the `git add` command.
- `commit`, `cmt`: Proxifies the `git commit` command.
- `checkout`, `co`: Proxifies the `git checkout` command.
- `branch`: Proxifies the `git branch` command.
- `merge`: Proxifies the `git merge` command.
- `pull`: Proxifies the `git pull` command.
- `push`: Proxifies the `git push` command.
- `log`: Proxifies the `git log` command.

#### Special Commands

- `mkbr`: Proxifies the `git checkout -b` command.