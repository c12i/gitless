# gitless

Gitless is a command-line interface (CLI) tool that facilitates cloning and degitting from a Git repository. It simplifies the process of cloning a repository and removing its Git history, leaving only the latest state of the code.

## installation

As cli

```sh
cargo install gitless
```

As lib

```sh
cargo add `gitless`
```


## cli usage

```
Usage: gitless [OPTIONS] --url <URL> --dest <DEST_PATH>

Options:
  -u, --url <URL>         Git URL you would like to clone and degit from
      --dest <DEST_PATH>  Destination of cloned and degitted path
  -b, --branch <BRANCH>   Git branch or tag to clone and degit from
  -r, --rev <REV>         Git commit hash to clone and degit from
  -h, --help              Print help
```

## notes

- If neither `--branch` nor `--rev` is provided, the tool will clone the default branch of the repository.
- Ensure that the specified destination path is valid and accessible.
- Use caution when using the `--rev`` option, as it will degit the repository at the specified commit, potentially resulting in a detached HEAD state.
- Gitless is not affiliated with Git or any Git hosting service. Use at your own discretion.
