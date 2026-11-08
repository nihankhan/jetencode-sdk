import json
from io import BytesIO
import unittest
from unittest.mock import patch
from urllib.error import HTTPError

from jetencode import APIError, Client


class Response:
    def __init__(self, data):
        self.data = data

    def __enter__(self):
        return self

    def __exit__(self, *_):
        pass

    def read(self):
        return self.data


class ClientTests(unittest.TestCase):
    def test_storage_serialization(self):
        source = {"service": "s3", "path": "platform/videos/movie.mp4"}
        default = {"source": source, "outputs": {"mp4:720p": {}}}
        override = {**default, "destination": {"output_directory": "custom/"}}
        credentials = {"region": "auto", "access_id": "legacy-id", "secret_key": "legacy-secret", "bucket": "legacy", "endpoint": "https://s3.test"}
        legacy = {"source": {**source, "credentials": credentials}, "outputs": {"mp4:720p": {}}, "destination": {**credentials, "output_directory": "out/"}}
        for request in (default, override, legacy):
            with self.subTest(request=request), patch("jetencode.client.urlopen", return_value=Response(b'{"id":"job","status":"queued"}')) as send:
                self.client.create_job(request)
                self.assertEqual(send.call_args.args[0].data, json.dumps(request).encode("utf-8"))
                self.assertEqual(json.loads(send.call_args.args[0].data), request)

    def setUp(self):
        self.client = Client("https://api.example.test/", "project", "key", timeout=5)

    def test_create_get_cancel(self):
        calls = []

        def send(request, timeout):
            calls.append((request, timeout))
            if len(calls) == 1:
                return Response(b'{"id":"job/1","status":"queued"}')
            if len(calls) == 2:
                return Response(b'{"data":{"id":"job/1","status":"completed"}}')
            return Response(b"")

        request = {"source": {"service": "http", "url": "https://example.com/video.mp4"}, "outputs": {"mp4:720p": {}}}
        with patch("jetencode.client.urlopen", side_effect=send):
            self.assertEqual(self.client.create_job(request)["status"], "queued")
            self.assertEqual(self.client.get_job("job/1")["status"], "completed")
            self.assertIsNone(self.client.cancel_job("job/1"))

        self.assertEqual([(req.get_method(), req.full_url) for req, _ in calls], [
            ("POST", "https://api.example.test/api/v1/job"),
            ("GET", "https://api.example.test/api/v1/job/job%2F1"),
            ("POST", "https://api.example.test/api/v1/job/job%2F1/cancel"),
        ])
        self.assertEqual(json.loads(calls[0][0].data), request)
        self.assertIsNone(calls[1][0].data)
        for req, timeout in calls:
            self.assertEqual(req.get_header("Authorization"), "Bearer key")
            self.assertEqual(req.get_header("X-project-id"), "project")
            self.assertEqual(req.get_header("Accept"), "application/json")
            self.assertEqual(timeout, 5)

    def test_api_error(self):
        error = HTTPError("https://api.example.test", 401, "Unauthorized", {}, BytesIO(b'{"message":"Unauthorized","error":"invalid key"}'))
        with patch("jetencode.client.urlopen", side_effect=error):
            with self.assertRaises(APIError) as ctx:
                self.client.get_job("job")
        self.assertEqual(ctx.exception.status_code, 401)
        self.assertEqual(ctx.exception.error_text, "invalid key")
        self.assertIn("Unauthorized", ctx.exception.body)

    def test_invalid_response_and_input(self):
        with patch("jetencode.client.urlopen", return_value=Response(b'{"data":null}')):
            with self.assertRaisesRegex(ValueError, "invalid job response"):
                self.client.get_job("job")
        with self.assertRaisesRegex(ValueError, "job ID is required"):
            self.client.cancel_job("")
        with self.assertRaisesRegex(ValueError, "api_key is required"):
            Client("https://api.example.test", "project", "")


if __name__ == "__main__":
    unittest.main()
