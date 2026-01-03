# Zinit

![Licenses](https://github.com/yonasBSD/rust-ci-github-actions-workflow/actions/workflows/licenses.yaml/badge.svg)
![Linting](https://github.com/yonasBSD/rust-ci-github-actions-workflow/actions/workflows/lint.yaml/badge.svg)
![Testing](https://github.com/yonasBSD/rust-ci-github-actions-workflow/actions/workflows/test-with-coverage.yaml/badge.svg)
![Packaging](https://github.com/yonasBSD/rust-ci-github-actions-workflow/actions/workflows/release-packaging.yaml/badge.svg)
![Cross-Build](https://github.com/yonasBSD/rust-ci-github-actions-workflow/actions/workflows/cross-build.yaml/badge.svg)

![Security Audit](https://github.com/yonasBSD/rust-ci-github-actions-workflow/actions/workflows/security.yaml/badge.svg)
![Scorecard Audit](https://github.com/yonasBSD/rust-ci-github-actions-workflow/actions/workflows/scorecard.yaml/badge.svg)
[![Quality Gate Status](https://sonarcloud.io/api/project_badges/measure?project=yonasBSD_rust-ci-github-actions-workflow&metric=alert_status)](https://sonarcloud.io/summary/new_code?id=yonasBSD_rust-ci-github-actions-workflow)
[![Security Rating](https://sonarcloud.io/api/project_badges/measure?project=yonasBSD_rust-ci-github-actions-workflow&metric=security_rating)](https://sonarcloud.io/summary/new_code?id=yonasBSD_rust-ci-github-actions-workflow)
[![Vulnerabilities](https://sonarcloud.io/api/project_badges/measure?project=yonasBSD_rust-ci-github-actions-workflow&metric=vulnerabilities)](https://sonarcloud.io/summary/new_code?id=yonasBSD_rust-ci-github-actions-workflow)
<!--[![codecov](https://codecov.io/gh/yonasBSD/rust-ci-github-actions-workflow/branch/main/graph/badge.svg?token=SLIHSUWHT2)](https://codecov.io/gh/yonasBSD/rust-ci-github-actions-workflow)-->
<!--[![ghcr.io](https://img.shields.io/badge/ghcr.io-download-blue)](https://github.com/yonasBSD/rust-ci-github-actions-workflow/pkgs/container/rust-ci-github-actions-workflow)-->
<!--[![Docker Pulls](https://img.shields.io/docker/pulls/rust-ci-github-actions-workflow/example.svg)](https://hub.docker.com/r/rust-ci-github-actions-workflow/example)-->
<!--[![Quay.io](https://img.shields.io/badge/Quay.io-download-blue)](https://quay.io/repository/rust-ci-github-actions-workflow/example)-->

![GitHub last commit](https://img.shields.io/github/last-commit/yonasBSD/rust-ci-github-actions-workflow)
[![Dependency Status](https://deps.rs/repo/github/yonasBSD/rust-ci-github-actions-workflow/status.svg)](https://deps.rs/repo/github/yonasBSD/rust-ci-github-actions-workflow)
![Rust](https://img.shields.io/badge/Built%20With-Rust-orange?logo=rust)
[![GitHub Release](https://img.shields.io/github/release/yonasBSD/rust-ci-github-actions-workflow.svg)](https://github.com/yonasBSD/rust-ci-github-actions-workflow/releases/latest)
[![License](https://img.shields.io/github/license/yonasBSD/rust-ci-github-actions-workflow.svg)](https://github.com/yonasBSD/rust-ci-github-actions-workflow/blob/main/LICENSE.txt)
<!--[![Matrix Chat](https://img.shields.io/matrix/vaultwarden:matrix.org.svg?logo=matrix)](https://matrix.to/#/#vaultwarden:matrix.org)-->

Zinit is a lightweight PID 1 replacement inspired by runit, written in Rust using Tokio for async I/O. It provides both a Unix socket interface and an HTTP API for interacting with the process manager.

### Key Features

- **Service Management**: Ensures configured services are up and running at all times
- **Dependency Handling**: Supports service dependencies for proper startup ordering
- **Simple Control Interface**: Provides an intuitive CLI to add, start, stop, and monitor services
- **Container Support**: Can run in container mode with appropriate signal handling
- **Configurable Logging**: Multiple logging options including ringbuffer and stdout

## Installation

```bash
curl https://raw.githubusercontent.com/threefoldtech/zinit/refs/heads/master/install.sh | bash

#to install & run
curl https://raw.githubusercontent.com/threefoldtech/zinit/refs/heads/master/install_run.sh | bash
```

Click [here](docs/installation.md) for more information on how to install Zinit.

## Usage

### Process Manager (zinit)

```bash
# Run zinit in init mode
zinit init --config /tmp/zinit/ --socket /var/run/zinit.sock

# List services
zinit list

# Start a service
zinit start <service-name>

# Stop a service
zinit stop <service-name>
```

```bash
# Start the HTTP proxy on the default port (8080)
zinit proxy
```

More information about all the available commands can be found [here](docs/cmd.md).

### Service Configuration

Zinit uses YAML files for service configuration. Here's a basic example:

```yaml
# Service configuration (e.g., /tmp/zinit/myservice.yaml)
exec: "/usr/bin/myservice --option value"   # Command to run (required)
test: "/usr/bin/check-myservice"            # Health check command (optional)
oneshot: false                              # Whether to restart on exit (default: false)
after:                                      # Services that must be running first (optional)
  - dependency1
  - dependency2
```

For more information on how to configure service files, see the [service file reference](docs/services.md) documentation.

### JSON-RPC API

The HTTP proxy provides a JSON-RPC 2.0 API for interacting with Zinit. You can send JSON-RPC requests to the HTTP endpoint you provided to the proxy:

```bash
curl -X POST -H "Content-Type: application/json" -d '{"jsonrpc":"2.0","id":1,"method":"service_list","params":{}}' http://localhost:8080/
```

See the [OpenRPC specs](openrpc.json) for more information about available RPC calls to interact with Zinit.

## License

See [LICENSE](LICENSE) file for details.
