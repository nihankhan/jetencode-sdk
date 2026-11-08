"""Synchronous, dependency-free client for JetEncode job endpoints."""

import json
from urllib.error import HTTPError
from urllib.parse import quote
from urllib.request import Request, urlopen


class APIError(Exception):
    """An unsuccessful HTTP response from the API."""

    def __init__(self, status_code, body):
        self.status_code = status_code
        self.body = body
        self.error_text = ""
        self.api_message = ""
        try:
            payload = json.loads(body)
            if isinstance(payload, dict):
                self.error_text = payload.get("error") if isinstance(payload.get("error"), str) else ""
                self.api_message = payload.get("message") if isinstance(payload.get("message"), str) else ""
        except (ValueError, TypeError):
            pass
        detail = self.error_text or self.api_message or body.strip()
        super().__init__(f"jetencode: HTTP {status_code}" + (f": {detail}" if detail else ""))


class Client:
    """Client for creating, reading, and cancelling transcoding jobs.

    Job requests and responses are dictionaries using the API's JSON field names.
    """

    def __init__(self, base_url, project_id, api_key, timeout=30):
        for label, value in (("base_url", base_url), ("project_id", project_id), ("api_key", api_key)):
            if not isinstance(value, str) or not value.strip():
                raise ValueError(f"jetencode: {label} is required")
        if not isinstance(timeout, (int, float)) or isinstance(timeout, bool) or timeout <= 0:
            raise ValueError("jetencode: timeout must be a positive number of seconds")
        self.base_url = base_url.rstrip("/")
        self.project_id = project_id
        self.api_key = api_key
        self.timeout = timeout

    def create_job(self, request):
        """Submit a job and return its JSON representation."""
        return self._job(self._request("POST", "/api/v1/job", request))

    def get_job(self, job_id):
        """Retrieve a job by ID."""
        return self._job(self._request("GET", self._job_path(job_id)))

    def cancel_job(self, job_id):
        """Request cancellation of a job (returns None on success)."""
        self._request("POST", self._job_path(job_id) + "/cancel", expect_job=False)

    @staticmethod
    def _job_path(job_id):
        if not isinstance(job_id, str) or not job_id.strip():
            raise ValueError("jetencode: job ID is required")
        return "/api/v1/job/" + quote(job_id, safe="")

    @staticmethod
    def _job(payload):
        if isinstance(payload, dict) and "data" in payload:
            payload = payload["data"]
        if not isinstance(payload, dict) or not isinstance(payload.get("id"), str) or not payload["id"] or not isinstance(payload.get("status"), str):
            raise ValueError("jetencode: invalid job response")
        return payload

    def _request(self, method, path, body=None, expect_job=True):
        data = json.dumps(body).encode("utf-8") if body is not None else None
        req = Request(
            self.base_url + path,
            data=data,
            method=method,
            headers={
                "Accept": "application/json",
                "Content-Type": "application/json",
                "X-Project-ID": self.project_id,
                "Authorization": "Bearer " + self.api_key,
            },
        )
        try:
            with urlopen(req, timeout=self.timeout) as response:
                raw = response.read().decode("utf-8")
        except HTTPError as exc:
            with exc:
                raw = exc.read().decode("utf-8", errors="replace")
            raise APIError(exc.code, raw) from exc
        if not expect_job:
            return None
        try:
            return json.loads(raw)
        except ValueError as exc:
            raise ValueError("jetencode: server returned invalid JSON") from exc
