# STM32 / Embassy

Use the MCU layer without the default Linux/std integrations:

```toml
[dependencies]
alumy = { path = "../..", default-features = false, features = ["embassy"] }
```

For firmware crates that need allocation-backed helpers, enable `embassy` and
`alumy-embassy/alloc` through a direct dependency on `alumy-embassy` or by
adding a focused feature in the application crate.
