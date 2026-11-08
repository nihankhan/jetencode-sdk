# JetEncode Python SDK

Synchronous Python 3.9+ client for the JetEncode job API. No runtime dependencies.

## Install

```bash
pip install ./sdk/python
```

## Example

```python
import os
from jetencode import APIError, Client

client = Client(
    base_url="https://your-jetencode-host",
    project_id="your-project-id",
    api_key=os.environ["JETENCODE_API_KEY"],
)

try:
    job = client.create_job({
        "source": {"service": "s3", "path": "platform/videos/movie.mp4"},
        "outputs": {"mp4:720p": {}},
    })
    print(client.get_job(job["id"]))
    # client.cancel_job(job["id"])
except APIError as exc:
    print(exc.status_code, exc.error_text, exc.body)
```

`create_job(request)` posts to `/api/v1/job`, `get_job(job_id)` reads
`/api/v1/job/{id}`, and `cancel_job(job_id)` posts to `/api/v1/job/{id}/cancel`.
Job results are dictionaries with the API's original JSON field names; both
direct responses and `{"data": {...}}` envelopes are supported. Requests use
`X-Project-ID` and `Authorization: Bearer <api_key>` headers. Set `timeout`
in seconds (default: 30). HTTP errors raise `APIError` with `status_code`,
`error_text`, `api_message`, and the raw response `body`. Transport errors
propagate from `urllib`.

Run tests from `sdk/python` with `PYTHONPATH=src python3 -m unittest discover -s tests`.

Omit destination for project storage, or send `{"output_directory": "transcoded/"}`
as destination to override only the prefix. Inline credentials remain supported.
See [default and legacy examples](../STORAGE.md#python).
