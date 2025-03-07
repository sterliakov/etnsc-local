etnsc-local is a tool for bootstrapping [Electroneum](https://github.com/electroneum/electroneum-sc)
local chain for development.

This is my submission to [Electroneum hackathon 2025](https://electroneum-hackathon-2025.devpost.com/).

## Prerequisites

This tool should work on Linux, MacOS and Windows. The only requirement is
having docker with compose plugin installed. If you don't have them available
yet, go through the [installation](https://docs.docker.com/engine/install/) guide. Docker binary should be available
on your `PATH`.

## Features

* No more `go build` hassle and local filesystem management!
* Start, stop and recreate the node in one command.
* Pre-seed accounts when node is launched - no need to do it manually.
* All your team members will run exactly the same reproducible setup.
* Forget about long builds and setups: runs in your favorite CI tool as well!
* *In progress: install from npm or PyPI.*

## Quickstart

To begin, grab a binary for your architecture from the Releases page.

First, run `init` to create a docker-compose file. You can tweak it as needed
and add other services. All configuration is done via environment variables
documented in that file.

Now you can launch the node. By default it provides HTTP API (port 8545)
and websocket API (port 8546), you can change that if necessary. You can
reach it at the `electroneum-node` host (from other containers) and directly
on localhost from your host system.

If you are already using compose, you can change the file name with `-f` flag
(to be provided to every subcommand) or with `ETNSC_COMPOSE_FILE` environmental
variable.

```bash
$ etnsc-local init
$ etnsc-local start
```

Read help for more:

```bash
$ etnsc-local help
```

## Installation

You can just download a binary from the Releases page:

```bash
$ wget https://FIXME
```

`cargo install` also works, if you have the Rut toolchain available:

```bash
$ cargo install etnsc-local
```
