<?php

declare(strict_types=1);

namespace JetEncode;

interface TransportInterface
{
    /**
     * @return array{status:int,body:string}
     */
    public function request(string $method, string $url, array $headers, ?string $body): array;
}
