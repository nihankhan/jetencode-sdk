use jetencode::{Client, Config, CreateJobRequest, Error, Response, Source, Transport};
use std::collections::BTreeMap;
use std::sync::Mutex;

struct MockTransport {
    status: u16,
    body: String,
    last_method: Mutex<String>,
    last_url: Mutex<String>,
}

impl MockTransport {
    fn new(status: u16, body: &str) -> Self {
        Self {
            status,
            body: body.to_string(),
            last_method: Mutex::new(String::new()),
            last_url: Mutex::new(String::new()),
        }
    }
}

impl Transport for MockTransport {
    fn request(
        &self,
        method: &str,
        url: &str,
        _headers: &[(String, String)],
        _body: Option<&str>,
    ) -> Result<Response, Error> {
        *self.last_method.lock().unwrap() = method.to_string();
        *self.last_url.lock().unwrap() = url.to_string();
        Ok(Response {
            status: self.status,
            body: self.body.clone(),
        })
    }
}

#[test]
fn create_job() {
    let transport = MockTransport::new(
        201,
        r#"{"message":"created","data":{"id":"job-1","status":"queued"}}"#,
    );
    let client = Client::new(Config {
        base_url: "http://example.com".into(),
        project_id: "proj".into(),
        api_key: "key".into(),
        timeout: std::time::Duration::from_secs(5),
    })
    .unwrap()
    .with_transport(Box::new(transport));

    let mut outputs = BTreeMap::new();
    outputs.insert("mp4:720p".into(), "{}".into());
    let req = CreateJobRequest {
        source: Source {
            service: "http".into(),
            url: Some("https://example.com/v.mp4".into()),
            ..Default::default()
        },
        outputs,
        destination: None,
        webhook: None,
    };
    let job = client.create_job(&req).unwrap();
    assert_eq!(job.id, "job-1");
    assert_eq!(job.status, "queued");
}

#[test]
fn get_job_direct() {
    let transport = MockTransport::new(200, r#"{"id":"job-2","status":"completed"}"#);
    let client = Client::new(Config {
        base_url: "http://example.com".into(),
        project_id: "proj".into(),
        api_key: "key".into(),
        timeout: std::time::Duration::from_secs(5),
    })
    .unwrap()
    .with_transport(Box::new(transport));
    let job = client.get_job("job-2").unwrap();
    assert_eq!(job.id, "job-2");
    assert_eq!(job.status, "completed");
}

#[test]
fn cancel_job() {
    let transport = MockTransport::new(202, r#"{"message":"Job cancelled successfully"}"#);
    let client = Client::new(Config {
        base_url: "http://example.com".into(),
        project_id: "proj".into(),
        api_key: "key".into(),
        timeout: std::time::Duration::from_secs(5),
    })
    .unwrap()
    .with_transport(Box::new(transport));
    client.cancel_job("job-3").unwrap();
}

#[test]
fn api_error() {
    let transport = MockTransport::new(
        401,
        r#"{"message":"Unauthorized","error":"invalid project id or api key"}"#,
    );
    let client = Client::new(Config {
        base_url: "http://example.com".into(),
        project_id: "proj".into(),
        api_key: "key".into(),
        timeout: std::time::Duration::from_secs(5),
    })
    .unwrap()
    .with_transport(Box::new(transport));
    let err = client.get_job("job-4").unwrap_err();
    match err {
        Error::Api { status, error, .. } => {
            assert_eq!(status, 401);
            assert_eq!(error, "invalid project id or api key");
        }
        _ => panic!("expected api error"),
    }
}
