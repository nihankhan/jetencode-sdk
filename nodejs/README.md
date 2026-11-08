# JetEncode Node.js SDK

The JetEncode Node.js SDK provides a typed client for submitting transcoding jobs, retrieving job status, and requesting job cancellation.

## Requirements

- Node.js 18 or newer
- A JetEncode API host
- A project ID
- An API key

## Installation

### Install from the local SDK directory

```bash
cd sdk/nodejs
npm install
npm run build
```

### Install from npm

After publishing the package to npm:

```bash
npm install jetencode-node-sdk
```

## Quick Start

```typescript
import { Client } from "jetencode-node-sdk";

const client = new Client({
  baseURL: "https://your-jetencode-host",
  projectID: "your-project-id",
  apiKey: process.env.JETENCODE_API_KEY!,
});

const job = await client.CreateJob({
  source: {
    service: "s3",
    path: "platform/videos/movie.mp4",
  },
  outputs: {
    "mp4:720p": {},
  },
});

console.log(job.id, job.status);
```

Set your API key in the environment:

```bash
export JETENCODE_API_KEY="your-api-key"
```

## Configuration

Omitted destination uses project storage. To override only the prefix, set
`destination: {output_directory: "transcoded/"}`. Existing inline credential
requests remain supported: [default and legacy examples](../STORAGE.md#typescript--nodejs).

| Option | Required | Description |
|---|---|---|
| `baseURL` | Yes | JetEncode API base URL |
| `projectID` | Yes | Project identifier |
| `apiKey` | Yes | API key used for Bearer authentication |
| `timeoutMs` | No | Request timeout in milliseconds; defaults to `30000` |
| `fetch` | No | Custom Fetch implementation |

## API Reference

### `new Client(config)`

Creates a client instance and validates its configuration.

### `CreateJob(request)`

Submits a transcoding job.

```typescript
const job = await client.CreateJob({
  source: {
    service: "http",
    url: "https://example.com/input/video.mp4",
  },
  outputs: {
    "mp4:720p": {},
  },
});
```

Returns a `Promise<Job>`.

### `GetJob(jobID)`

Retrieves an existing job by ID.

```typescript
const job = await client.GetJob("your-job-id");
console.log(job.status);
```

Returns a `Promise<Job>`.

### `CancelJob(jobID)`

Requests cancellation of an existing job.

```typescript
await client.CancelJob("your-job-id");
```

Returns a `Promise<void>`. Successful completion means the cancellation request was accepted by the API; it does not necessarily mean processing has already stopped.

## Error Handling

Non-success HTTP responses throw an `APIError`.

```typescript
import { APIError } from "jetencode-node-sdk";

try {
  const job = await client.GetJob("your-job-id");
  console.log(job);
} catch (error: unknown) {
  if (error instanceof APIError) {
    console.error("HTTP status:", error.statusCode);
    console.error("API error:", error.errorText);
    console.error("Response body:", error.body);
  } else {
    console.error("Request failed:", error);
  }
}
```

`APIError` exposes:

- `statusCode`: HTTP status code
- `errorText`: API's `error` field, when present
- `message`: Human-readable error message
- `body`: Raw response body

Network failures, timeouts, and invalid JSON responses are reported separately.

## Authentication

The SDK sends these headers with each request:

- `Authorization: Bearer <API_KEY>`
- `X-Project-ID: <PROJECT_ID>`
- `Accept: application/json`
- `Content-Type: application/json`

Keep API keys in environment variables or a secrets manager. Do not commit credentials to source control.

## Supported Operations

| Operation | HTTP method | Endpoint |
|---|---|---|
| Create job | POST | `/api/v1/job` |
| Get job | GET | `/api/v1/job/{jobID}` |
| Cancel job | POST | `/api/v1/job/{jobID}/cancel` |

The SDK supports job responses returned directly or wrapped in a `data` envelope.

## Development

```bash
npm install
npm run build
npm run example
```

The example requires a reachable JetEncode API host and valid credentials.

## License

MIT
