<?php

declare(strict_types=1);

namespace JetEncode;

class CurlTransport implements TransportInterface
{
    private int $timeout;

    public function __construct(int $timeout = 30)
    {
        $this->timeout = $timeout;
    }

    public function request(string $method, string $url, array $headers, ?string $body): array
    {
        $ch = curl_init($url);
        curl_setopt($ch, CURLOPT_CUSTOMREQUEST, $method);
        curl_setopt($ch, CURLOPT_RETURNTRANSFER, true);
        curl_setopt($ch, CURLOPT_TIMEOUT, $this->timeout);
        $headerLines = [];
        foreach ($headers as $k => $v) {
            $headerLines[] = $k . ': ' . $v;
        }
        curl_setopt($ch, CURLOPT_HTTPHEADER, $headerLines);
        if ($body !== null) {
            curl_setopt($ch, CURLOPT_POSTFIELDS, $body);
        }
        $respBody = curl_exec($ch);
        if ($respBody === false) {
            $err = curl_error($ch);
            curl_close($ch);
            throw new \RuntimeException('jetencode: transport error: ' . $err);
        }
        $status = (int) curl_getinfo($ch, CURLINFO_HTTP_CODE);
        curl_close($ch);
        return ['status' => $status, 'body' => (string) $respBody];
    }
}
