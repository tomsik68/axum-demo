# axum-demo

This repo showcases a simple Rust web service using the [axum](https://lib.rs/axum) web framework.

The service additionally has:

- [tracing](https://lib.rs/tracing) for logging
- [opentelemetry](https://opentelemetry.io) observability
- [degeneric-macros](https://lib.rs/degeneric-macros) dependency container
- [kubernetes](https://kubernetes.io) deployment (without opentelemetry collector)
- [skaffold](https://skaffold.dev) configuration

# Requirements

- [Docker](https://docker.com) (for k3d to work in skaffold)
- [k3d](https://k3d.io)
- [skaffold](https://skaffold.dev)
- [kubectl](https://kubernetes.io)

# Running

```bash
$ make dev
$ curl localhost:8080
```
