# JetEncode Rust SDK

Synchronous typed client with injectable `Transport` and explicit JSON serialization.
Use an S3 `Source` with only `service` and `path`; set `destination: None` for
project defaults, or `Some(StorageConfig { output_directory, ..Default::default() })`
for a directory override. Existing inline structs remain supported unchanged.

See [complete default and legacy examples](../STORAGE.md#rust), including HTTPS
transport requirements. Run `cargo test` from this directory.
