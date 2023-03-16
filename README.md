# CloudEvents Pretty Printer WASM

## About

This library implements a pretty printer for CloudEvents. The output will be
similar to the following:

```
☁️  cloudevents.Event
Validation: valid
Context Attributes,
  specversion: 1.0
  type: dev.knative.cli.plugin.event.generic
  source: kn-event/v1.6.0
  id: 60fd4b51-510c-4ce4-a058-e91704c74e6d
  time: 2023-03-15T19:48:58.710062181Z
  datacontenttype: application/json
Extensions,
  traceparent: 00-0af7651916cd43dd8448eb211c80319c-b9c7c989f97918e1-01
Data,
  {
    "a": {
      "b": {
        "c": "hello"
      }
    }
  }
```

## 🚴 Usage

TBD

### 🛠️ Build

Make sure you have `cargo-make` installed. You can install it by running:

```asciidoc
cargo install --no-default-features --force cargo-make
```

then call simply:

```
cargo make
```

### 🔬 Test

```
cargo make test
```

### 🎁 Publish OCI registry

```
cargo make publish
```

**NOTE**: You need to set `REGISTRY_USERNAME` and `REGISTRY_PASSWORD`
environment variables. By default, the registry is set to
`ghcr.io/cardil/cloudevents-pretty-print`. You can change it by
setting `REGISTRY` environment variable.

## License

Licensed under

* Apache License, Version 2.0, ([LICENSE](LICENSE))

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be licensed with Apache-2.0 as well, without any additional
terms or conditions.
