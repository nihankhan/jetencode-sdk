# JetEncode Go SDK

Official Go client for the JetEncode transcoding API.

## Installation

```bash
go get github.com/GotipathTeam/jetencode/sdk/go
```

## Usage

```go
package main

import (
    "context"
    "fmt"

    jetencode "github.com/GotipathTeam/jetencode/sdk/go"
)

func main() {
    client, err := jetencode.NewClient(jetencode.Config{
        BaseURL:   "https://your-jetencode-host",
        ProjectID: "3fa85f64-5717-4562-b3fc-2c963f66afa6",
        APIKey:    "your-api-key",
    })
    if err != nil {
        panic(err)
    }

    job, err := client.CreateJob(context.Background(), jetencode.CreateJobRequest{
        Source: jetencode.Source{Service: "s3", Path: "platform/videos/movie.mp4"},
        Outputs: map[string]any{"mp4:720p": map[string]any{}},
    })
    if err != nil {
        panic(err)
    }
    fmt.Println(job.ID, job.Status)
}
```

## Supported operations

- `CreateJob(ctx, req)` — POST /api/v1/job
- `GetJob(ctx, jobID)` — GET /api/v1/job/:jobID
- `CancelJob(ctx, jobID)` — POST /api/v1/job/:jobID/cancel
## Storage

Omitting destination uses project S3 storage and its configured output directory.
Use `StorageConfig{OutputDirectory: "transcoded/"}` to override only the directory.
Legacy inline fields remain supported. See [default and legacy examples](../STORAGE.md#go).
