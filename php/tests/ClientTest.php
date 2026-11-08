<?php

declare(strict_types=1);

namespace JetEncode\Tests;

require_once __DIR__ . '/../src/TransportInterface.php';
require_once __DIR__ . '/../src/CurlTransport.php';
require_once __DIR__ . '/../src/ApiException.php';
require_once __DIR__ . '/../src/Models.php';
require_once __DIR__ . '/../src/Client.php';

use JetEncode\ApiException;
use JetEncode\Client;
use JetEncode\TransportInterface;

class MockTransport implements TransportInterface
{
    public array $lastRequest = [];
    public int $status = 200;
    public string $body = '{}';

    public function request(string $method, string $url, array $headers, ?string $body): array
    {
        $this->lastRequest = compact('method', 'url', 'headers', 'body');
        return ['status' => $this->status, 'body' => $this->body];
    }
}

function assertSame($a, $b, string $msg = ''): void
{
    if ($a !== $b) {
        throw new \RuntimeException($msg . " expected " . var_export($b, true) . " got " . var_export($a, true));
    }
}

// Test 1
$source = new \JetEncode\Source(service: 's3', path: 'platform/videos/movie.mp4');
$request = new \JetEncode\CreateJobRequest($source, ['mp4:720p' => new \stdClass()]);
assertSame(json_encode($request->toArray(), JSON_UNESCAPED_SLASHES), '{"source":{"service":"s3","path":"platform/videos/movie.mp4"},"outputs":{"mp4:720p":{}}}');
$request->destination = new \JetEncode\StorageConfig(outputDirectory: 'custom/');
assertSame(json_encode($request->toArray(), JSON_UNESCAPED_SLASHES), '{"source":{"service":"s3","path":"platform/videos/movie.mp4"},"outputs":{"mp4:720p":{}},"destination":{"output_directory":"custom/"}}');
$source->credentials = new \JetEncode\S3Credentials('auto', 'legacy-id', 'legacy-secret', 'legacy', 'https://s3.test');
$request->destination = new \JetEncode\StorageConfig('https://output.test', 'auto', 'output', 'out/', 'output-id', 'output-secret');
assertSame(json_encode($request->toArray(), JSON_UNESCAPED_SLASHES), '{"source":{"service":"s3","path":"platform/videos/movie.mp4","credentials":{"region":"auto","access_id":"legacy-id","secret_key":"legacy-secret","bucket":"legacy","endpoint":"https://s3.test"}},"outputs":{"mp4:720p":{}},"destination":{"endpoint":"https://output.test","region":"auto","bucket":"output","output_directory":"out/","access_id":"output-id","secret_key":"output-secret"}}');

$transport = new MockTransport();
$transport->status = 201;
$transport->body = json_encode(['message' => 'created', 'data' => ['id' => 'job-1', 'status' => 'queued']]);
$client = new Client(['base_url' => 'https://example.com', 'project_id' => 'proj', 'api_key' => 'key', 'transport' => $transport]);
$typed = new \JetEncode\CreateJobRequest(new \JetEncode\Source(service: 's3', path: 'movie.mp4'), ['mp4:720p' => new \stdClass()]);
$client->createJob($typed);
assertSame($transport->lastRequest['body'], '{"source":{"service":"s3","path":"movie.mp4"},"outputs":{"mp4:720p":{}}}');
$typed->destination = new \JetEncode\StorageConfig(outputDirectory: 'custom');
$client->createJob($typed);
assertSame($transport->lastRequest['body'], '{"source":{"service":"s3","path":"movie.mp4"},"outputs":{"mp4:720p":{}},"destination":{"output_directory":"custom"}}');
$job = $client->createJob(['source' => ['service' => 'http', 'url' => 'https://example.com/v.mp4'], 'outputs' => ['mp4:720p' => new \stdClass()]]);
assertSame('job-1', $job->id, 'id');
assertSame('queued', $job->status, 'status');
assertSame('POST', $transport->lastRequest['method'], 'method');
echo "PASS createJob\n";

// Test 2
$transport = new MockTransport();
$transport->status = 200;
$transport->body = json_encode(['id' => 'job-2', 'status' => 'completed']);
$client = new Client(['base_url' => 'https://example.com', 'project_id' => 'proj', 'api_key' => 'key', 'transport' => $transport]);
$job = $client->getJob('job-2');
assertSame('job-2', $job->id, 'id');
assertSame('completed', $job->status, 'status');
echo "PASS getJob\n";

// Test 3
$transport = new MockTransport();
$transport->status = 202;
$transport->body = json_encode(['message' => 'Job cancelled successfully']);
$client = new Client(['base_url' => 'https://example.com', 'project_id' => 'proj', 'api_key' => 'key', 'transport' => $transport]);
$client->cancelJob('job-3');
if (strpos($transport->lastRequest['url'], '/api/v1/job/job-3/cancel') === false) {
    throw new \RuntimeException('cancel url mismatch');
}
echo "PASS cancelJob\n";

// Test 4
$transport = new MockTransport();
$transport->status = 401;
$transport->body = json_encode(['message' => 'Unauthorized', 'error' => 'invalid project id or api key']);
$client = new Client(['base_url' => 'https://example.com', 'project_id' => 'proj', 'api_key' => 'key', 'transport' => $transport]);
try {
    $client->getJob('job-4');
    echo "FAIL apiError: no exception\n";
} catch (ApiException $e) {
    if ($e->getStatusCode() !== 401) {
        throw new \RuntimeException('status mismatch');
    }
    echo "PASS apiError\n";
}
