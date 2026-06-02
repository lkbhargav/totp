# TOTP CLI

A command-line TOTP (time-based one-time password) token generator based on
[RFC-6238](https://datatracker.ietf.org/doc/html/rfc6238). Accounts and their
secrets are stored in an AES-256-CBC encrypted file, unlocked by a password you
provide each time you run a command.

> **Note on names:** the project builds a binary called `gotp`, but the CLI
> presents its help under the name `totp`. Examples below use `gotp`. See
> [Aliases](#aliases) if you'd prefer to call it `totp`.

## Quick Start

```bash
# Clone and install
git clone https://github.com/Skarlso/totp.git
cd totp
make install          # builds and installs the `gotp` binary into ~/.cargo/bin

# Add your first account, then generate a code
gotp add                       # prompts for account name, BASE32 secret, password
gotp generate -a gmail_main    # prompts for password, prints a 6-digit code
```

## Prerequisites

- A recent Rust toolchain — the crate uses `edition = "2024"`, so install via
  [rustup](https://rustup.rs/) and keep it up to date.
- OpenSSL, required to build the `openssl` crate:
  - **macOS:** `brew install openssl` (usually already present).
  - **Debian/Ubuntu:** `sudo apt install libssl-dev pkg-config`.

## Install

Install the `gotp` binary into `~/.cargo/bin`:

```bash
make install
# or, equivalently:
cargo install --path .
```

To build locally instead of installing:

```bash
make build      # debug build
make release    # optimized build at target/release/gotp
```

## Security

This CLI never stores your password. It prompts for one on every command. The
password unlocks an AES-256-CBC encrypted file named `.account.txt`, which holds
all your account names and TOTP secrets. The file is created in the **current
working directory**, so run `gotp` from the same directory each time (a stable
location such as your home directory is a good choice).

`.account.txt` is git-ignored and should never be committed.

## Usage

```bash
❯ gotp help
TOTP Token generator on the command line with AES encrypted account handling.

Usage: gotp [COMMAND]

Commands:
  add       Adds a new account with a TOTP token.
  generate  Generate a new token for a given account.
  delete    Delete a given account.
  help      Print this message or the help of the given subcommand(s)
```

### Add an account

`add` is interactive. Provide a name for the account and its BASE32-encoded TOTP
secret (the string a service shows you when enabling 2FA), then your password.

```bash
❯ gotp add
account:
gmail_main
token:
MFZWIZQASDFFSFDSIJAAA=
Password:
```

### Generate a token

Generates a 6-digit code for the account, valid for 30 seconds.

```bash
❯ gotp generate -a gmail_main
Password:
364898
```

### Delete an account

```bash
❯ gotp delete -a gmail_main
Password:
```

## Tricks

### Copy a token straight to the clipboard

```bash
# macOS
gotp generate -a gmail_main | pbcopy

# Linux (X11 / Wayland)
gotp generate -a gmail_main | xclip -selection clipboard
gotp generate -a gmail_main | wl-copy
```

Enter your password when prompted, then paste the code wherever you need it.

### Aliases

If you have many accounts, typing the full name each time is tedious. Add helper
functions to your `.bashrc` / `.zshrc`:

```bash
gotp-generate() {
    gotp generate -a "$1"
}

gg()  { gotp-generate personal.gmail; }
wgg() { gotp-generate work.gmail; }
```

To call the binary as `totp`, add an alias:

```bash
alias totp='gotp'
```

## Development

Common tasks are wrapped in the [`Makefile`](./Makefile):

```bash
make help        # list all targets
make build       # debug build
make test        # run the test suite
make fmt         # format with rustfmt
make lint        # run clippy (fails on warnings)
make ci          # fmt-check + lint + test (run before committing)
```

The project ships with unit tests covering encryption, file handling, and token
generation. Run them with `make test` (or `cargo test`).

## Compliance with the RFC

This generator complies with [RFC-6238](https://datatracker.ietf.org/doc/html/rfc6238),
with the exception that the time step and HMAC algorithm are currently fixed
(30-second steps, HMAC-SHA1) and not yet configurable.

## Contributions

Contributions are welcome.
