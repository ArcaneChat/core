<p align="center">
  <img alt="Delta Chat Logo" height="200px" src="https://raw.githubusercontent.com/deltachat/deltachat-pages/master/assets/blog/rust-delta.png">
</p>

<p align="center">
  <a href="https://github.com/chatmail/core/actions/workflows/ci.yml">
    <img alt="Rust CI" src="https://github.com/chatmail/core/actions/workflows/ci.yml/badge.svg">
  </a>
  <a href="https://deps.rs/repo/github/chatmail/core">
    <img alt="dependency status" src="https://deps.rs/repo/github/chatmail/core/status.svg">
  </a>
</p>

<p align="center">
The core library for Delta Chat, written in Rust
</p>

## Installing Rust and Cargo

To download and install the official compiler for the Rust programming language, and the Cargo package manager, run the command in your user environment:

```
$ curl https://sh.rustup.rs -sSf | sh
```

> On Windows, you may need to also install **Perl** to be able to compile deltachat-core.

## Using the CLI client

Compile and run Delta Chat Core command line utility, using `cargo`:

```
$ cargo run --locked -p deltachat-repl -- ~/deltachat-db
```
where ~/deltachat-db is the database file. Delta Chat will create it if it does not exist.

Optionally, install `deltachat-repl` binary with
```
$ cargo install --locked --path deltachat-repl/
```
and run as
```
$ deltachat-repl ~/deltachat-db
```

Configure your account (if not already configured):

```
Delta Chat Core is awaiting your commands.
> set addr your@email.org
> set mail_pw yourpassword
> configure
```

Connect to your mail server (if already configured):

```
> connect
```

Create a contact:

```
> addcontact yourfriends@email.org
Command executed successfully.
```

List contacts:

```
> listcontacts
Contact#10: <name unset> <yourfriends@email.org>
Contact#1: Me √√ <your@email.org>
```

Create a chat with your friend and send a message:

```
> createchat 10
Single#10 created successfully.
> chat 10
Single#10: yourfriends@email.org [yourfriends@email.org]
> send hi
Message sent.
```

If `yourfriend@email.org` uses DeltaChat, but does not receive message just
sent, it is advisable to check `Spam` folder. It is known that at least
`gmx.com` treat such test messages as spam, unless told otherwise with web
interface.

List messages when inside a chat:

```
> chat
```

For more commands type:

```
> help
```

## Installing libdeltachat system wide

```
$ git clone https://github.com/chatmail/core.git
$ cd deltachat-core-rust
$ cmake -B build . -DCMAKE_INSTALL_PREFIX=/usr
$ cmake --build build
$ sudo cmake --install build
```

## Development

```sh
# run tests
$ cargo test --all
# build c-ffi
$ cargo build -p deltachat_ffi --release
```

## Debugging environment variables 

- `DCC_MIME_DEBUG`: if set outgoing and incoming message will be printed 

- `RUST_LOG=async_imap=trace,async_smtp=trace`: enable IMAP and
SMTP tracing in addition to info messages.

### Expensive tests

Some tests are expensive and marked with `#[ignore]`, to run these
use the `--ignored` argument to the test binary (not to cargo itself):
```sh
$ cargo test -- --ignored
```

### Fuzzing

Install [`cargo-bolero`](https://github.com/camshaft/bolero) with
```sh
$ cargo install cargo-bolero@0.8.0
```

Run fuzzing tests with
```sh
$ cd fuzz
$ cargo bolero test fuzz_mailparse --release=false -s NONE
```

Corpus is created at `fuzz/fuzz_targets/corpus`,
you can add initial inputs there.
For `fuzz_mailparse` target corpus can be populated with
`../test-data/message/*.eml`.

To run with AFL instead of libFuzzer:
```sh
$ cargo bolero test fuzz_format_flowed --release=false -e afl -s NONE
```

## Features

- `vendored`: When using Openssl for TLS, this bundles a vendored version.

## Update Provider Data

To add the updates from the
[provider-db](https://github.com/deltachat/provider-db) to the core, run:

```
./src/provider/update.py ../provider-db/_providers/ > src/provider/data.rs
```

## Language bindings and frontend projects

Language bindings are available for:

- **C** \[[📂 source](./deltachat-ffi) | [📚 docs](https://c.delta.chat)\]
- **JS**: \[[📂 source](./deltachat-rpc-client) | [📦 npm](https://www.npmjs.com/package/@deltachat/jsonrpc-client) | [📚 docs](https://js.jsonrpc.delta.chat/)\]
- **Python** \[[📂 source](./python) | [📦 pypi](https://pypi.org/project/deltachat) | [📚 docs](https://py.delta.chat)\]
- **Go**
  - over jsonrpc: \[[📂 source](https://github.com/deltachat/deltachat-rpc-client-go/)\]
  - over cffi[^1]: \[[📂 source](https://github.com/deltachat/go-deltachat/)\]
- **Free Pascal**[^1] \[[📂 source](https://github.com/deltachat/deltachat-fp/)\]
- **Java** and **Swift** (contained in the Android/iOS repos)

The following "frontend" projects make use of the Rust-library
or its language bindings:

- [Android](https://github.com/deltachat/deltachat-android)
- [iOS](https://github.com/deltachat/deltachat-ios)
- [Desktop](https://github.com/deltachat/deltachat-desktop)
- [Pidgin](https://code.ur.gs/lupine/purple-plugin-delta/)
- [Telepathy](https://code.ur.gs/lupine/telepathy-padfoot/)
- [Ubuntu Touch](https://codeberg.org/lk108/deltatouch)
- several **Bots**

[^1]: Out of date / unmaintained, if you like those languages feel free to start maintaining them. If you have questions we'll help you, please ask in the issues.
