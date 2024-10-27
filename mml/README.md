# 📫 mml-lib

Rust implementation of the *Emacs MIME message Meta Language*, as known as [MML](https://www.gnu.org/software/emacs/manual/html_node/emacs-mime/Composing.html).

## Features

- Compiles MML to MIME messages
- Interprets MIME messages as MML
- Supports multiple parts `<#multipart>…<#/multipart>`
- Supports inline part `<#part text=mime/type>…<#/part>`
- Supports attachment `<#part disposition=attachment filename=/path/to/attachment.ext><#/part>`
- Supports comment `<#!part>This will not be compiled<#!/part>`
- Supports **tokio** and **async-std** async runtimes
- Supports **rustls** and **native-tls** crypto libs
- Supports **PGP**: shell commands, GPG bindings or native implem with [`pgp-lib`](https://crates.io/crates/pgp-lib)
- Retrieves PGP secret keys and passphrases from shell commands or global keyring via [`secret-lib`](https://crates.io/crates/secret-lib)
- Supports **serde** (de)serialization

The library comes with 13 [cargo features](https://doc.rust-lang.org/cargo/reference/features.html), including 4 default ones:

- **`tokio`**: enables the [tokio](https://crates.io/crates/tokio) async runtime
- `async-std`: enables the [async-std](https://crates.io/crates/async-std) async runtime
- **`rustls`**: enables the [rustls](https://crates.io/crates/rustls) crypto
- `native-tls`: enables the [native-tls](https://crates.io/crates/native-tls) crypto
- **`compiler`**: enables MML to MIME compilation
- **`interpreter`**: enables MIME to MML interpretation
- `pgp-commands`: enables PGP using [shell commands](https://crates.io/crates/process-lib)
- `pgp-gpg`: enables PGP using [GPG bindings](https://crates.io/crates/gpgme)
- `pgp-native`: enables native PGP using [`pgp-lib`](https://crates.io/crates/pgp-lib)
- `command`: enables command-based [secrets](https://crates.io/crates/secret-lib) for `pgp-native`
- `keyring`: enables keyring-based [secrets](https://crates.io/crates/secret-lib) for `pgp-native`
- `derive`: enables [serde](https://crates.io/crates/serde) support
- `vendored`: compiles and statically link to a copy of non-Rust vendors like OpenSSL

## Definition

From the [Emacs documentation](https://www.gnu.org/software/emacs/manual/html_node/emacs-mime/MML-Definition.html):

> Creating a MIME message is boring and non-trivial. Therefore, a library called mml has been defined that parses a language called MML (MIME Meta Language) and generates MIME messages.
>
> The MML language is very simple. It looks a bit like an SGML application, but it’s not.
> 
> The main concept of MML is the part. Each part can be of a different type or use a different charset. The way to delineate a part is with a ‘<#part ...>’ tag. Multipart parts can be introduced with the ‘<#multipart ...>’ tag. Parts are ended by the ‘<#/part>’ or ‘<#/multipart>’ tags. Parts started with the ‘<#part ...>’ tags are also closed by the next open tag.
> 
> […]
> 
> Each tag can contain zero or more parameters on the form ‘parameter=value’. The values may be enclosed in quotation marks, but that’s not necessary unless the value contains white space. So ‘filename=/home/user/#hello$^yes’ is perfectly valid.

## Examples

```eml
From: alice@localhost
To: bob@localhost
Subject: MML examples

This is a plain text part.

<#part type=text/html>
<h1>This is a HTML part.</h1>
<#/part>

<#part description="This is an attachment." filename=./examples/attachment.png><#/part>
```

compiles to:

```eml
MIME-Version: 1.0
From: <alice@localhost>
To: <bob@localhost>
Subject: MML examples
Message-ID: <17886a741feef4a2.f9706245cd3a3f97.3b41d60ef9e2fbfb@soywod>
Date: Tue, 26 Sep 2023 09:58:26 +0000
Content-Type: multipart/mixed; 
	boundary="17886a741fef2cb2_97a7dbff4c84bbac_3b41d60ef9e2fbfb"


--17886a741fef2cb2_97a7dbff4c84bbac_3b41d60ef9e2fbfb
Content-Type: text/plain; charset="utf-8"
Content-Transfer-Encoding: 7bit

This is a plain text part.


--17886a741fef2cb2_97a7dbff4c84bbac_3b41d60ef9e2fbfb
Content-Type: text/html; charset="utf-8"
Content-Transfer-Encoding: 7bit

<h1>This is a HTML part.</h1>

Content-Type: application/octet-stream
Content-Disposition: attachment; filename="attachment.png"
Content-Transfer-Encoding: base64

iVBORw0KGgo…

--17886a741fef2cb2_97a7dbff4c84bbac_3b41d60ef9e2fbfb--
```

Other examples can be found at [`./examples`](https://github.com/pimalaya/core/tree/master/mml/examples):

```sh
cargo run --example
```

*See the full API documentation on [docs.rs](https://docs.rs/mml-lib/latest/mml/).*

## Sponsoring

[![nlnet](https://nlnet.nl/logo/banner-160x60.png)](https://nlnet.nl/)

Special thanks to the [NLnet foundation](https://nlnet.nl/) and the [European Commission](https://www.ngi.eu/) that helped the project to receive financial support from various programs:

- [NGI Assure](https://nlnet.nl/project/Himalaya/) in 2022
- [NGI Zero Entrust](https://nlnet.nl/project/Pimalaya/) in 2023
- [NGI Zero Core](https://nlnet.nl/project/Pimalaya-PIM/) in 2024 *(still ongoing)*

If you appreciate the project, feel free to donate using one of the following providers:

[![GitHub](https://img.shields.io/badge/-GitHub%20Sponsors-fafbfc?logo=GitHub%20Sponsors)](https://github.com/sponsors/soywod)
[![Ko-fi](https://img.shields.io/badge/-Ko--fi-ff5e5a?logo=Ko-fi&logoColor=ffffff)](https://ko-fi.com/soywod)
[![Buy Me a Coffee](https://img.shields.io/badge/-Buy%20Me%20a%20Coffee-ffdd00?logo=Buy%20Me%20A%20Coffee&logoColor=000000)](https://www.buymeacoffee.com/soywod)
[![Liberapay](https://img.shields.io/badge/-Liberapay-f6c915?logo=Liberapay&logoColor=222222)](https://liberapay.com/soywod)
[![thanks.dev](https://img.shields.io/badge/-thanks.dev-000000?logo=data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iMjQuMDk3IiBoZWlnaHQ9IjE3LjU5NyIgY2xhc3M9InctMzYgbWwtMiBsZzpteC0wIHByaW50Om14LTAgcHJpbnQ6aW52ZXJ0IiB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciPjxwYXRoIGQ9Ik05Ljc4MyAxNy41OTdINy4zOThjLTEuMTY4IDAtMi4wOTItLjI5Ny0yLjc3My0uODktLjY4LS41OTMtMS4wMi0xLjQ2Mi0xLjAyLTIuNjA2di0xLjM0NmMwLTEuMDE4LS4yMjctMS43NS0uNjc4LTIuMTk1LS40NTItLjQ0Ni0xLjIzMi0uNjY5LTIuMzQtLjY2OUgwVjcuNzA1aC41ODdjMS4xMDggMCAxLjg4OC0uMjIyIDIuMzQtLjY2OC40NTEtLjQ0Ni42NzctMS4xNzcuNjc3LTIuMTk1VjMuNDk2YzAtMS4xNDQuMzQtMi4wMTMgMS4wMjEtMi42MDZDNS4zMDUuMjk3IDYuMjMgMCA3LjM5OCAwaDIuMzg1djEuOTg3aC0uOTg1Yy0uMzYxIDAtLjY4OC4wMjctLjk4LjA4MmExLjcxOSAxLjcxOSAwIDAgMC0uNzM2LjMwN2MtLjIwNS4xNTYtLjM1OC4zODQtLjQ2LjY4Mi0uMTAzLjI5OC0uMTU0LjY4Mi0uMTU0IDEuMTUxVjUuMjNjMCAuODY3LS4yNDkgMS41ODYtLjc0NSAyLjE1NS0uNDk3LjU2OS0xLjE1OCAxLjAwNC0xLjk4MyAxLjMwNXYuMjE3Yy44MjUuMyAxLjQ4Ni43MzYgMS45ODMgMS4zMDUuNDk2LjU3Ljc0NSAxLjI4Ny43NDUgMi4xNTR2MS4wMjFjMCAuNDcuMDUxLjg1NC4xNTMgMS4xNTIuMTAzLjI5OC4yNTYuNTI1LjQ2MS42ODIuMTkzLjE1Ny40MzcuMjYuNzMyLjMxMi4yOTUuMDUuNjIzLjA3Ni45ODQuMDc2aC45ODVabTE0LjMxNC03LjcwNmgtLjU4OGMtMS4xMDggMC0xLjg4OC4yMjMtMi4zNC42NjktLjQ1LjQ0NS0uNjc3IDEuMTc3LS42NzcgMi4xOTVWMTQuMWMwIDEuMTQ0LS4zNCAyLjAxMy0xLjAyIDIuNjA2LS42OC41OTMtMS42MDUuODktMi43NzQuODloLTIuMzg0di0xLjk4OGguOTg0Yy4zNjIgMCAuNjg4LS4wMjcuOTgtLjA4LjI5Mi0uMDU1LjUzOC0uMTU3LjczNy0uMzA4LjIwNC0uMTU3LjM1OC0uMzg0LjQ2LS42ODIuMTAzLS4yOTguMTU0LS42ODIuMTU0LTEuMTUydi0xLjAyYzAtLjg2OC4yNDgtMS41ODYuNzQ1LTIuMTU1LjQ5Ny0uNTcgMS4xNTgtMS4wMDQgMS45ODMtMS4zMDV2LS4yMTdjLS44MjUtLjMwMS0xLjQ4Ni0uNzM2LTEuOTgzLTEuMzA1LS40OTctLjU3LS43NDUtMS4yODgtLjc0NS0yLjE1NXYtMS4wMmMwLS40Ny0uMDUxLS44NTQtLjE1NC0xLjE1Mi0uMTAyLS4yOTgtLjI1Ni0uNTI2LS40Ni0uNjgyYTEuNzE5IDEuNzE5IDAgMCAwLS43MzctLjMwNyA1LjM5NSA1LjM5NSAwIDAgMC0uOTgtLjA4MmgtLjk4NFYwaDIuMzg0YzEuMTY5IDAgMi4wOTMuMjk3IDIuNzc0Ljg5LjY4LjU5MyAxLjAyIDEuNDYyIDEuMDIgMi42MDZ2MS4zNDZjMCAxLjAxOC4yMjYgMS43NS42NzggMi4xOTUuNDUxLjQ0NiAxLjIzMS42NjggMi4zNC42NjhoLjU4N3oiIGZpbGw9IiNmZmYiLz48L3N2Zz4=)](https://thanks.dev/soywod)
[![PayPal](https://img.shields.io/badge/-PayPal-0079c1?logo=PayPal&logoColor=ffffff)](https://www.paypal.com/paypalme/soywod)
