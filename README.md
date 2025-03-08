etnsc-bootstrap is a tool for bootstrapping [Electroneum](https://github.com/electroneum/electroneum-sc)
local chain for development.

This is my submission to [Electroneum hackathon 2025](https://electroneum-hackathon-2025.devpost.com/).

## Prerequisites

This tool should work on Linux, MacOS and Windows. The only requirement is
having docker with compose installed. If you don't have them available
yet, go through the [installation](https://docs.docker.com/engine/install/) guide. Docker binary should be available
on your `PATH`.

## Features

* No more `go build` hassle and local filesystem management!
* Start, stop and recreate the node in one command.
* Pre-seed accounts when node is launched - no need to do it manually.
* All your team members will run exactly the same reproducible setup.
* Forget about long builds and setups: runs in your favorite CI tool as well!
* Install from npm - no need to configure unfamiliar build tools and compilers.

## Quickstart

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
$ npx etnsc-bootstrap init
$ npx etnsc-bootstrap start
```

Re-seed the node (remove the data and begin from scratch):

```bash
$ npx etnsc-bootstrap reset
```

Stop the node when no longer needed:

```bash
$ npx etnsc-bootstrap stop
```

Read help for more:

```bash
$ npx etnsc-bootstrap help
```

## Installation

The easiest way is to use `npx` if you have it installed:

```bash
$ npx etnsc-bootstrap init
$ npx etnsc-bootstrap start
```

You can just download a binary for your OS and arch from the Releases page, unpack and run it. `cargo install` also works, if you have the Rust toolchain available:

```bash
$ cargo install etnsc-bootstrap
```

## Remix users

The node started this way can be used as a drop-in replacement for a manually
started `etn-sc`. Same caveats apply: if you consider a web-based IDE usable
and want to use Remix, you need to allow CORS requests to the node.

To do so, edit the compose file created by `etnsc-bootstrap init` to add Remix
to whitelisted domains. This is controlled by `HTTP_CORS_HOSTS` (or `CORS_HOSTS`
for HTTP and websocket APIs together) variable. Adjust the file as follows:

```yaml
# You can edit this file freely, add other services, adjust the values
# or do something else.
services:
    # Please do not rename this service. If necessary, add hostname
    # for inter-container communication if `electroneum-node` is too long
    # for you.
    electroneum-node:
        image: sterliakov/electroneum-local:latest
        # You can use command to pass any extra flags to etn-sc
        command: ''
        environment:
            # What domains to allow for websocket and HTTP (CORS) connections?
            CORS_HOSTS: ''
            # What domains to allow for HTTP (CORS) connections? Overrides CORS_HOSTS
            HTTP_CORS_HOSTS: 'http://remix.ethereum.org' # <<< EDIT this line
            # ...
# Rest of the file unchanged
```
