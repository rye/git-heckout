# git-heckout

## What is it?

`git-heckout` lists your branches and fuzzy matches your given branch name so
that you can check out branches without typing their names entirely in.

For example:

```console
$ git heckout ma
Successfully checked out branch master.
```

## Installation

You can install/update git-heckout via:

```console
$ cargo install --force git-heckout
```

or, via `cargo install --path /path/to/clone`.  You may also find it useful to
have an alias, e.g. `git config --global alias.co heckout`, or by adding

```
[alias]
        co = heckout
```

to your gitconfig.  This allows you to use the shorthand

```console
$ git co ma
```

which internally executes `git checkout master` if `master` is a branch.

## Status

All of the basic functionality is effectively working, but a number of features
have not been added yet.
