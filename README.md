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

##### `sync`

See the full documentation [here](cmds/sync/README.md).

##### `br`

Checkout/create branches interactively (by selecting from a list of branches).

```bash
$ br 
? Select a branch to checkout
> master
  feature/branch
  bugfix/branch
  hotfix/branch
```

```bash
$ br feat/branch
# checkout or create and checkout the branch 'feat/branch'
```

##### `git usr`

Manage global Git user settings (`user.name`, `user.email`, `user.signingkey`), allowing us to
switch between a predefined set of profiles interactively.

```bash
# add a new profile
$ git usr add # or gusr add
> id 0x
> name 0x
> email 0x@testing.com
> key Q1W2E3R4T5Y6U7I8
```

```bash
# list all available profiles
$ git usr list # or gusr list
   id        name            email               gpg         
 ────────────────────────────────────────────────────────── 
  0x     0x             0x@testing.com     Q1W2E3R4T5Y6U7I8  
  lucas  Lucas Colombo  lucas@testing.com  8I7U6Y5T4R3E2W1Q  
 ────────────────────────────────────────────────────────── 
```

```bash
# set the active profile
$ git usr set # or gusr set
? Select a profile to set as active
> 0x
  lucas

Active profile set to: 0x <0x@testing.com>
```

```bash
# show current active profile
$ git usr # or gusr
Name: Lucas Colombo
Email: lucasncolombo@gmail.com
Signing key: Q1W2E3R4T5Y6U7I8
```

```bash
# show the entire configuration file
$ git usr cfg # or gusr cfg
─────┬────────────────────────────────────────────────
     │ File: /home/lucas/.config/gitools/config.toml
─────┼────────────────────────────────────────────────
   1 │ [[profile]]
   2 │ id = "0x"
   3 │ name = "lu0x"
   4 │ email = "0x@testing.co"
   5 │ signingkey = "Q1W2E3R4T5Y6U7I8"
   6 │
   7 │ [[profile]]
   8 │ id = "lucas"
   9 │ name = "Lucas Colombo"
  10 │ email = "lucas@testing.co"
  11 │ signingkey = "8I7U6Y5T4R3E2W1Q"
─────┴────────────────────────────────────────────────
```

```bash
# remove a profile
$ git usr rm # or gusr rm
? Select a profile to remove
> 0x
  lucas
```
