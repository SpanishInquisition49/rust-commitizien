# rs-commitizien

`rs-commitizien` is a command-line tool written in Rust for validating commit
messages according to the Conventional Commit standard. It helps ensure
consistency and clarity in commit messages across a project, making it easier
to understand the purpose of each commit.

## Installation

To install `rs-commitizien`, you'll need Rust and Cargo installed on your
system. Once you have them installed, you can use Cargo to install
`rs-commitizien`:

```bash
cargo install rs-commitizien

```

## Usage

After installation, you can use rs-commitizien to validate commit messages.
Simply navigate to your project directory and run:

```bash
rs-commitizien <commit-message>
```

Replace \<commit-message\> with the commit message you want to validate. The
tool will output whether the commit message conforms to the Conventional Commit
standard.

## Conventional Commit Standard

The Conventional Commit standard provides a set of rules for formatting commit
messages. Each commit message consists of a header, an optional body, and an
optional footers. The header has a specific format:

```bash
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

Where:

- `type`: Describes the kind of change being made (e.g., feat, fix, docs).
- `scope` (optional): Specifies the scope of the change (e.g., component,
  module).
- `description`: Provides a brief summary of the change.

See more at [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/).

## Examples

Here's an example of a valid commit message:

```plaintext
feat: allow provided config object to extend other configs

BREAKING CHANGE: `extends` key in config file is now used for extending other config files
```
