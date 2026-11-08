use jetencode::{CreateJobRequest, Source, StorageConfig, S3Credentials};
use std::collections::BTreeMap;

#[test]
fn default_and_legacy_storage_json() {
    let mut outputs = BTreeMap::new();
    outputs.insert("mp4:720p".to_string(), "{}".to_string());
    let mut request = CreateJobRequest {
        source: Source { service: "s3".into(), path: Some("platform/videos/movie.mp4".into()), ..Default::default() },
        outputs, destination: None, webhook: None,
    };
    assert_eq!(request.to_json(), r#"{"source":{"service":"s3","path":"platform/videos/movie.mp4"},"outputs":{"mp4:720p":{}}}"#);
    request.destination = Some(StorageConfig { output_directory: "custom/".into(), ..Default::default() });
    assert_eq!(request.to_json(), r#"{"source":{"service":"s3","path":"platform/videos/movie.mp4"},"outputs":{"mp4:720p":{}},"destination":{"output_directory":"custom/"}}"#);
    request.source.credentials = Some(S3Credentials { region:"auto".into(), access_id:"legacy-id".into(), secret_key:"legacy-secret".into(), bucket:"legacy".into(), endpoint:"https://s3.test".into() });
    request.destination = Some(StorageConfig { endpoint:"https://output.test".into(), region:"auto".into(), bucket:"output".into(), output_directory:"out/".into(), access_id:"output-id".into(), secret_key:"output-secret".into() });
    assert_eq!(request.to_json(), r#"{"source":{"service":"s3","path":"platform/videos/movie.mp4","credentials":{"region":"auto","access_id":"legacy-id","secret_key":"legacy-secret","bucket":"legacy","endpoint":"https://s3.test"}},"outputs":{"mp4:720p":{}},"destination":{"endpoint":"https://output.test","region":"auto","bucket":"output","output_directory":"out/","access_id":"output-id","secret_key":"output-secret"}}"#);
}
