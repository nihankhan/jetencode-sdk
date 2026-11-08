import assert from "node:assert/strict";
import { test } from "node:test";
import { APIError, Client } from "../dist/src/index.js";

test("project defaults, directory overrides, and legacy storage serialize exactly", async () => {
  const bodies = [];
  const client = new Client({baseURL: "https://api.example.test", projectID: "project", apiKey: "key",
    fetch: async (_, options) => { bodies.push(options.body); return new Response('{"id":"job","status":"queued"}'); }});
  const source = {service: "s3", path: "platform/videos/movie.mp4"};
  await client.CreateJob({source, outputs: {"mp4:720p": {}}});
  assert.equal(bodies[0], '{"source":{"service":"s3","path":"platform/videos/movie.mp4"},"outputs":{"mp4:720p":{}}}');
  await client.CreateJob({source, outputs: {"mp4:720p": {}}, destination: {output_directory: "custom/"}});
  assert.equal(bodies[1], '{"source":{"service":"s3","path":"platform/videos/movie.mp4"},"outputs":{"mp4:720p":{}},"destination":{"output_directory":"custom/"}}');
  const credentials = {region:"auto",access_id:"legacy-id",secret_key:"legacy-secret",bucket:"legacy",endpoint:"https://s3.test"};
  const legacy = {source: {...source, credentials}, outputs:{"mp4:720p":{}}, destination:{...credentials,output_directory:"out/"}};
  await client.CreateJob(legacy);
  assert.equal(bodies[2], JSON.stringify(legacy));
  assert.equal(JSON.parse(bodies[2]).source.credentials.secret_key, "legacy-secret");
});

test("create, get, cancel: headers, payload, paths and envelopes", async () => {
  const calls = [];
  const client = new Client({
    baseURL: "https://api.example.test/",
    projectID: "project",
    apiKey: "key",
    fetch: async (url, options) => {
      calls.push({ url, options });
      return new Response(calls.length === 3 ? null : calls.length === 2
        ? JSON.stringify({ data: { id: "job/1", status: "completed" } })
        : calls.length === 1
          ? JSON.stringify({ id: "job/1", status: "queued" })
          : "", { status: calls.length === 3 ? 204 : 200 });
    },
  });
  const request = { source: { service: "http", url: "https://example.com/video.mp4" }, outputs: { "mp4:720p": {} } };
  assert.deepEqual(await client.CreateJob(request), { id: "job/1", status: "queued" });
  assert.deepEqual(await client.GetJob("job/1"), { id: "job/1", status: "completed" });
  await client.CancelJob("job/1");
  assert.deepEqual(calls.map(({ url, options }) => [options.method, url]), [
    ["POST", "https://api.example.test/api/v1/job"],
    ["GET", "https://api.example.test/api/v1/job/job%2F1"],
    ["POST", "https://api.example.test/api/v1/job/job%2F1/cancel"],
  ]);
  assert.deepEqual(JSON.parse(calls[0].options.body), request);
  for (const { options } of calls) {
    assert.equal(options.headers.Authorization, "Bearer key");
    assert.equal(options.headers["X-Project-ID"], "project");
    assert.equal(options.headers.Accept, "application/json");
  }
  assert.equal(calls[1].options.body, undefined);
});

test("API errors expose status and server details", async () => {
  const client = new Client({
    baseURL: "https://api.example.test", projectID: "project", apiKey: "key",
    fetch: async () => new Response(JSON.stringify({ message: "Unauthorized", error: "invalid key" }), { status: 401 }),
  });
  await assert.rejects(client.GetJob("job"), (error) => {
    assert.ok(error instanceof APIError);
    assert.equal(error.statusCode, 401);
    assert.equal(error.errorText, "invalid key");
    assert.match(error.body, /Unauthorized/);
    return true;
  });
});

test("invalid job responses are rejected", async () => {
  const client = new Client({
    baseURL: "https://api.example.test", projectID: "project", apiKey: "key",
    fetch: async () => new Response(JSON.stringify({ data: null }), { status: 200 }),
  });
  await assert.rejects(client.GetJob("job"), /missing its ID/);
  await assert.rejects(client.GetJob(""), /job ID is required/);
});
