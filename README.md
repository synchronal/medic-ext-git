# Medic Git

An extension pack for using [medic](https://github.com/synchronal/medic-rs)
with git.

## Installation

```shell
brew tap synchronal/tap
brew install medic-ext-git
```

Example `Brewfile`:

```shell
tap 'synchronal/tap'

brew  'synchronal/tap/medic'
brew  'synchronal/tap/medic-ext-git'
```

## Usage

```toml
[doctor]

checks = []

[test]

checks = []

[audit]

checks = []

[update]

steps = [
  { step = "git", command = "pull" },
]

[shipit]

steps = [
  { audit = {} },
  { update = {} },
  { test = {} },
  { step = "git", command = "push" },
  { step = "github", command = "link-to-actions", verbose = true },
]
```


## medic-stp-git

Steps for interacting with git.

### pull

Pull the current repository with `--rebase`.

```shell
medic-step-git pull
```

### push

Push the current repository to `origin`.

```shell
medic-step-git push
```


## medic-step-github

Steps for helping with repositories that are hosted in GitHub.

### link-to-actions

Using the git remote configured for `--remote`, print out the URL
for its GitHub Actions. Defaults to `origin`.

```shell
medic-step-github link-to-actions
medic-step-github link-to-actions --remote <remote>
```

