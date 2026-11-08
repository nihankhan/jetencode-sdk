<?php

declare(strict_types=1);

namespace JetEncode;

class S3Credentials
{
    public function __construct(
        public string $region = '',
        public string $accessId = '',
        public string $secretKey = '',
        public string $bucket = '',
        public string $endpoint = ''
    ) {
    }

    public function toArray(): array
    {
        return [
            'region' => $this->region,
            'access_id' => $this->accessId,
            'secret_key' => $this->secretKey,
            'bucket' => $this->bucket,
            'endpoint' => $this->endpoint,
        ];
    }
}

class Source
{
    public function __construct(
        public string $service = '',
        public ?string $url = null,
        public ?string $path = null,
        public ?S3Credentials $credentials = null,
        public ?string $endpoint = null,
        public ?string $region = null,
        public ?string $accessId = null,
        public ?string $secretKey = null,
        public ?string $bucket = null
    ) {
    }

    public function toArray(): array
    {
        $out = ['service' => $this->service];
        if ($this->url !== null) {
            $out['url'] = $this->url;
        }
        if ($this->path !== null) {
            $out['path'] = $this->path;
        }
        if ($this->credentials !== null) {
            $out['credentials'] = $this->credentials->toArray();
        }
        if ($this->endpoint !== null) {
            $out['endpoint'] = $this->endpoint;
        }
        if ($this->region !== null) {
            $out['region'] = $this->region;
        }
        if ($this->accessId !== null) {
            $out['access_id'] = $this->accessId;
        }
        if ($this->secretKey !== null) {
            $out['secret_key'] = $this->secretKey;
        }
        if ($this->bucket !== null) {
            $out['bucket'] = $this->bucket;
        }
        return $out;
    }
}

class StorageConfig
{
    public function __construct(
        public string $endpoint = '',
        public string $region = '',
        public string $bucket = '',
        public string $outputDirectory = '',
        public string $accessId = '',
        public string $secretKey = ''
    ) {
    }

    public function toArray(): array
    {
        if ($this->endpoint === '' && $this->region === '' && $this->bucket === '' && $this->accessId === '' && $this->secretKey === '') {
            return $this->outputDirectory === '' ? [] : ['output_directory' => $this->outputDirectory];
        }
        return [
            'endpoint' => $this->endpoint,
            'region' => $this->region,
            'bucket' => $this->bucket,
            'output_directory' => $this->outputDirectory,
            'access_id' => $this->accessId,
            'secret_key' => $this->secretKey,
        ];
    }
}

class TranscoderWebhook
{
    public function __construct(public string $webhookUrl = '')
    {
    }

    public function toArray(): array
    {
        return ['webhook_url' => $this->webhookUrl];
    }
}

class CreateJobRequest
{
    public function __construct(
        public Source $source,
        public array $outputs = [],
        public ?StorageConfig $destination = null,
        public ?TranscoderWebhook $webhook = null
    ) {
    }

    public function toArray(): array
    {
        $out = [
            'source' => $this->source->toArray(),
            'outputs' => $this->outputs,
        ];
        if ($this->destination !== null) {
            $destination = $this->destination->toArray();
            if ($destination !== []) {
                $out['destination'] = $destination;
            }
        }
        if ($this->webhook !== null) {
            $out['webhook'] = $this->webhook->toArray();
        }
        return $out;
    }
}

class OutputDetail
{
    public ?string $name = null;
    public ?string $path = null;
    public ?string $status = null;
    public ?string $duration = null;
    public ?string $resolution = null;
    public ?string $signedUrl = null;
    public ?string $videoCodec = null;
    public ?string $audioCodec = null;

    public static function fromArray(array $data): self
    {
        $o = new self();
        $o->name = $data['name'] ?? null;
        $o->path = $data['path'] ?? null;
        $o->status = $data['status'] ?? null;
        $o->duration = $data['duration'] ?? null;
        $o->resolution = $data['resolution'] ?? null;
        $o->signedUrl = $data['signed_url'] ?? null;
        $o->videoCodec = $data['video_codec'] ?? null;
        $o->audioCodec = $data['audio_codec'] ?? null;
        return $o;
    }
}

class Job
{
    public ?string $id = null;
    public ?string $projectId = null;
    public ?string $status = null;
    public ?string $filename = null;
    public int $downloadProgress = 0;
    public int $uploadProgress = 0;
    public ?string $videoCodec = null;
    public ?string $audioCodec = null;
    public ?string $frameRate = null;
    public ?string $bitrate = null;
    public ?string $colorSpace = null;
    public ?string $sourceService = null;
    public array $outputFormats = [];
    public array $outputDetails = [];

    public static function fromArray(array $data): self
    {
        $j = new self();
        $j->id = $data['id'] ?? null;
        $j->projectId = $data['project_id'] ?? null;
        $j->status = $data['status'] ?? null;
        $j->filename = $data['filename'] ?? null;
        $j->downloadProgress = $data['download_progress'] ?? 0;
        $j->uploadProgress = $data['upload_progress'] ?? 0;
        $j->videoCodec = $data['video_codec'] ?? null;
        $j->audioCodec = $data['audio_codec'] ?? null;
        $j->frameRate = $data['frame_rate'] ?? null;
        $j->bitrate = $data['bitrate'] ?? null;
        $j->colorSpace = $data['color_space'] ?? null;
        $j->sourceService = $data['source_service'] ?? null;
        $j->outputFormats = $data['output_formats'] ?? [];
        foreach ($data['output_details'] ?? [] as $od) {
            $j->outputDetails[] = OutputDetail::fromArray($od);
        }
        return $j;
    }
}
