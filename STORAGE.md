# Storage in every SDK

All five SDKs support project defaults and the existing inline credential format.
Use a server with [project-default storage support](../docs/default-storage.md)
before deploying path-only requests. Authentication remains the project API key
and project ID configured on the client.

## Go

```go
request := jetencode.CreateJobRequest{
    Source: jetencode.Source{Service: "s3", Path: "platform/videos/movie.mp4"},
    Outputs: map[string]any{"mp4:720p": map[string]any{}},
}
// Optional project-storage directory override:
request.Destination = jetencode.StorageConfig{OutputDirectory: "transcoded/"}
// client.CreateJob(ctx, request)

// Legacy overrides remain supported; source and destination may differ.
request.Source.Credentials = &jetencode.S3Credentials{
    Endpoint: "https://input.example.com", Region: "auto", Bucket: "input",
    AccessID: inputAccessID, SecretKey: inputSecret,
}
request.Destination = jetencode.StorageConfig{
    Endpoint: "https://output.example.com", Region: "auto", Bucket: "output",
    AccessID: outputAccessID, SecretKey: outputSecret, OutputDirectory: "transcoded/",
}
```

## TypeScript / Node.js

```typescript
import type { CreateJobRequest } from "jetencode-node-sdk";
const request: CreateJobRequest = {
  source: {service: "s3", path: "platform/videos/movie.mp4"},
  outputs: {"mp4:720p": {}}
};
request.destination = {output_directory: "transcoded/"}; // optional
// await client.CreateJob(request);

// Legacy:
request.source.credentials = {
  endpoint: "https://input.example.com", region: "auto", bucket: "input",
  access_id: inputAccessID, secret_key: inputSecret
};
request.destination = {
  endpoint: "https://output.example.com", region: "auto", bucket: "output",
  access_id: outputAccessID, secret_key: outputSecret, output_directory: "transcoded/"
};
```

## Python

```python
request = {
    "source": {"service": "s3", "path": "platform/videos/movie.mp4"},
    "outputs": {"mp4:720p": {}}
}
request["destination"] = {"output_directory": "transcoded/"}  # optional
# client.create_job(request)

# Legacy:
request["source"]["credentials"] = {
    "endpoint": "https://input.example.com", "region": "auto", "bucket": "input",
    "access_id": input_access_id, "secret_key": input_secret
}
request["destination"] = {
    "endpoint": "https://output.example.com", "region": "auto", "bucket": "output",
    "access_id": output_access_id, "secret_key": output_secret,
    "output_directory": "transcoded/"
}
```

## PHP

```php
$request = new \JetEncode\CreateJobRequest(
    new \JetEncode\Source(service: 's3', path: 'platform/videos/movie.mp4'),
    ['mp4:720p' => new \stdClass()]
);
$request->destination = new \JetEncode\StorageConfig(outputDirectory: 'transcoded/'); // optional
// $client->createJob($request);

// Legacy (existing positional and named constructor arguments still work):
$request->source->credentials = new \JetEncode\S3Credentials(
    region: 'auto', accessId: $inputAccessID, secretKey: $inputSecret,
    bucket: 'input', endpoint: 'https://input.example.com'
);
$request->destination = new \JetEncode\StorageConfig(
    endpoint: 'https://output.example.com', region: 'auto', bucket: 'output',
    outputDirectory: 'transcoded/', accessId: $outputAccessID, secretKey: $outputSecret
);
```

## Rust

```rust
use jetencode::{CreateJobRequest, Source, StorageConfig, S3Credentials};
use std::collections::BTreeMap;
let mut outputs = BTreeMap::new();
outputs.insert("mp4:720p".into(), "{}".into());
let mut request = CreateJobRequest {
    source: Source { service: "s3".into(), path: Some("platform/videos/movie.mp4".into()), ..Default::default() },
    outputs, destination: None, webhook: None,
};
request.destination = Some(StorageConfig { output_directory: "transcoded/".into(), ..Default::default() });
// client.create_job(&request)

// Legacy:
request.source.credentials = Some(S3Credentials {
    endpoint: "https://input.example.com".into(), region: "auto".into(), bucket: "input".into(),
    access_id: input_access_id, secret_key: input_secret,
});
request.destination = Some(StorageConfig {
    endpoint: "https://output.example.com".into(), region: "auto".into(), bucket: "output".into(),
    access_id: output_access_id, secret_key: output_secret, output_directory: "transcoded/".into(),
});
```

Rust's current built-in transport supports HTTP only. For HTTPS deployments use
`Client::with_transport` with a TLS-capable `Transport`; do not send project keys
over public plaintext HTTP. This release changes storage serialization, not transport.

Defaults omit credential fields. A Go zero destination or PHP `null`/empty typed
destination is omitted; a Rust `None` destination is omitted (`Some(Default::default())`
serializes `{}`, which has the same backend default meaning). Dictionary/object
SDKs transmit exactly what the caller supplies, so omit unused credential fields.
Existing flat source fields and full inline destination serialization remain supported.
