# Description

This project consists of two binaries:
- `validate-agentd` - gRPC service, which validates User-Agent strings
- `validate-agent` - CLI client used to interact with `validate-agentd`

## `validate-agentd`

The service can be started like so:

```sh
$ validate-agentd
```

It will serve requests until Ctrl-C signal is received on command line.

By default the server listens on `[::]:50000`, but this can be overriden by using `--addr` flag, for example

```sh
$ validate-agentd --addr 127.0.0.1:8080
```

## `validate-agent`

With `validate-agentd` running on a local system or accessible via network, `validate-agent` can be used to interact with it, for example:

```sh
$ validate-agent "Mozilla/5.0 (Windows NT 6.1; Win64; x64; rv:47.0) Gecko/20100101 Firefox/47.0"
 INFO validate_agent: User-Agent allowed
```

By default the assumes `validate-agentd` to be accessible on `[::1]:50000`, but this can be overriden by using `--addr` flag, for example

```sh
$ validate-agent --addr 127.0.0.1:8080 "Mozilla/5.0 (Windows NT 6.1; Win64; x64; rv:47.0) Gecko/20100101 Firefox/47.0"
```

The tool is designed to work well in both interactive and non-interactive use cases. Here is an example script, which could be used on command-line
 to validate User-Agent headers of a real HTTP client:

```sh
$ validate-agent "$(nc -w 1 -l 50001 | grep User-Agent | cut -d ' ' -f2-)"
```

# Development

## Dependencies

Project depends upon a recent stable Rust toolchain and `protoc`.

### Nix

A Nix development shell is provided, which includes all the required tooling, use `nix develop` to access it.

## Build

Once all dependencies are available, `cargo build` can be used to build the project.

# Installation

## Cargo

Use:

```sh
$ cargo install --git https://github.com/rvolosatovs/validate-agent
```
to install the project directly from this Git repository.

## Nix

Use:

```sh
$ nix shell github:rvolosatovs/validate-agent
```

To get `validate-agent` and `validate-agentd` available in your environment without installing


If you prefer to install the tools to your profile, use:

```sh
$ nix profile install github:rvolosatovs/validate-agent
```
