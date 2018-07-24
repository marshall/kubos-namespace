Namespacing KubOS APIs
---

This is a proof of concept to show how we could use the combination of module re-export, Cargo
optional dependencies, and Cargo "features" to deliver a unified `kubos` API namespace to our users.

In this PoC, each module that should goes under the `kubos` top level namespace is implemented as
a crate named `kubos-<namespace>`, [imported conditionally](apis/kubos/src/lib.rs) by the `kubos`
crate, and finally is added as an optional dependency and a matching feature in the `kubos`
[Cargo.toml](apis/kubos/Cargo.toml).

Features instead of dependencies for Apps
---

A KubOS app will now just need to list each "feature" that corresponds to a top level namespace in
it's dependency on the top level `kubos` crate, i.e:

```toml
[dependencies]
kubos = { version = "0.1.0", features = ["app", "telemetry"] }
```

By listing `telemetry` as a feature, the KubOS App only needs to declare the `kubos` crate as extern,
but can now use the `kubos::app` namespace which is simply the re-exported `kubos_app` crate.

See code in [example-app/src/main.rs](examples/example-app/src/main.rs)
