# axum-demo

This repo showcases a simple Rust web service using the [axum](https://lib.rs/axum) web framework.

The service additionally has:

- [tracing](https://lib.rs/tracing) for logging
- [opentelemetry](https://opentelemetry.io) observability
- [degeneric-macros](https://lib.rs/degeneric-macros) dependency container
- [helm](https://helm.sh) chart
- [tilt](https://tilt.dev) configuration (including opentelemetry collector!)

# Requirements

- [Docker](https://docker.com) (for k3d to work)
- [k3d](https://k3d.io)
- [tilt](https://tilt.dev)
- [kubectl](https://kubernetes.io)

# Running

```bash
$ tilt up
$ curl localhost:8080
```
