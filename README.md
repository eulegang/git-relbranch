# git-relbranch

Find the relative position of Git branches compared to a reference branch.

`git-relbranch` lists branches in the current repository and shows how many commits each branch is ahead of and behind the reference branch, along with the branch tip author and timestamp.

## Usage

Run from anywhere inside a Git repository:

```sh
git-relbranch [OPTIONS]
```

Options:

```text
-a, --all                    Show remote branches as well
-r, --reference <REFERENCE>  The base branch to compare with
-h, --help                   Print help
```

By default, the current `HEAD` branch is used as the reference and only local branches are shown.

## Example

```text
+3 -1 feature/login  Alice  2024-02-10T12:34:56+00:00
+0 -5 main           Bob    2024-02-09T09:15:00+00:00
```

In each row:

- `+N` is the number of commits the listed branch has that the reference does not.
- `-N` is the number of commits the reference has that the listed branch does not.
- The remaining columns are branch name, last commit author, and last commit time.

## Installation

### From source

```sh
git clone <repo-url>
cd git-relbranch
cargo install --path .
```

Or build a local debug binary:

```sh
cargo build
./target/debug/git-relbranch --help
```

## Development

This project is written in Rust and uses Cargo.

```sh
cargo build
cargo run -- --help
```

## License

MIT
