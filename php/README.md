# JetEncode PHP SDK

PHP 8.1+ client with typed models or raw array requests. Configure `base_url`,
`project_id`, and `api_key` on `JetEncode\Client`.

Use `new Source(service: 's3', path: 'platform/videos/movie.mp4')` and omit
destination for project storage. A `StorageConfig(outputDirectory: 'transcoded/')`
overrides only the directory. Existing inline constructors remain supported.

See [complete default and legacy examples](../STORAGE.md#php).
Run `php tests/ClientTest.php` from this directory.
