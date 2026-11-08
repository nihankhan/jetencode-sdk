<?php

declare(strict_types=1);

namespace JetEncode;

class Client
{
    private string $baseUrl;
    private string $projectId;
    private string $apiKey;
    private TransportInterface $transport;

    public function __construct(array $config)
    {
        if (empty($config['base_url'])) {
            throw new \InvalidArgumentException('jetencode: base_url is required');
        }
        if (empty($config['project_id'])) {
            throw new \InvalidArgumentException('jetencode: project_id is required');
        }
        if (empty($config['api_key'])) {
            throw new \InvalidArgumentException('jetencode: api_key is required');
        }
        $this->baseUrl = rtrim($config['base_url'], '/');
        $this->projectId = $config['project_id'];
        $this->apiKey = $config['api_key'];
        $this->transport = $config['transport'] ?? new CurlTransport($config['timeout'] ?? 30);
    }

    public function createJob(CreateJobRequest|array $request): Job
    {
        $payload = $request instanceof CreateJobRequest ? $request->toArray() : $request;
        $data = $this->request('POST', '/api/v1/job', $payload);
        return Job::fromArray($this->extractData($data));
    }

    public function getJob(string $jobId): Job
    {
        $data = $this->request('GET', '/api/v1/job/' . rawurlencode($jobId), null);
        return Job::fromArray($this->extractData($data));
    }

    public function cancelJob(string $jobId): void
    {
        $this->request('POST', '/api/v1/job/' . rawurlencode($jobId) . '/cancel', null);
    }

    private function request(string $method, string $path, ?array $body): array
    {
        $headers = [
            'Content-Type' => 'application/json',
            'Accept' => 'application/json',
            'X-Project-ID' => $this->projectId,
            'Authorization' => 'Bearer ' . $this->apiKey,
        ];
        $payload = $body === null ? null : json_encode($body);
        $res = $this->transport->request($method, $this->baseUrl . $path, $headers, $payload);
        $decoded = json_decode($res['body'], true);
        if ($res['status'] < 200 || $res['status'] >= 300) {
            $message = is_array($decoded) ? ($decoded['message'] ?? '') : '';
            $error = is_array($decoded) ? ($decoded['error'] ?? '') : '';
            throw new ApiException($res['status'], $message, $error, $res['body']);
        }
        return is_array($decoded) ? $decoded : [];
    }

    private function extractData(array $data): array
    {
        if (isset($data['data']) && is_array($data['data'])) {
            return $data['data'];
        }
        return $data;
    }
}
