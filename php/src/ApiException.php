<?php

declare(strict_types=1);

namespace JetEncode;

class ApiException extends \RuntimeException
{
    private int $statusCode;
    private string $responseBody;
    private string $serverError;

    public function __construct(int $statusCode, string $message, string $serverError = '', string $responseBody = '')
    {
        parent::__construct(sprintf('jetencode: HTTP %d: %s', $statusCode, $serverError !== '' ? $serverError : $message));
        $this->statusCode = $statusCode;
        $this->serverError = $serverError;
        $this->responseBody = $responseBody;
    }

    public function getStatusCode(): int
    {
        return $this->statusCode;
    }

    public function getServerError(): string
    {
        return $this->serverError;
    }

    public function getResponseBody(): string
    {
        return $this->responseBody;
    }
}
