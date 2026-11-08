use std::collections::BTreeMap;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct S3Credentials {
    pub region: String,
    pub access_id: String,
    pub secret_key: String,
    pub bucket: String,
    pub endpoint: String,
}

impl S3Credentials {
    pub fn to_json(&self) -> String {
        format!(
            r#"{{"region":"{}","access_id":"{}","secret_key":"{}","bucket":"{}","endpoint":"{}"}}"#,
            escape(&self.region),
            escape(&self.access_id),
            escape(&self.secret_key),
            escape(&self.bucket),
            escape(&self.endpoint)
        )
    }
}

#[derive(Debug, Clone, Default)]
pub struct Source {
    pub service: String,
    pub url: Option<String>,
    pub path: Option<String>,
    pub credentials: Option<S3Credentials>,
    // flat compatibility fields
    pub endpoint: Option<String>,
    pub region: Option<String>,
    pub access_id: Option<String>,
    pub secret_key: Option<String>,
    pub bucket: Option<String>,
}

impl Source {
    pub fn to_json(&self) -> String {
        let mut parts = vec![format!(r#""service":"{}""#, escape(&self.service))];
        if let Some(v) = &self.url {
            parts.push(format!(r#""url":"{}""#, escape(v)));
        }
        if let Some(v) = &self.path {
            parts.push(format!(r#""path":"{}""#, escape(v)));
        }
        if let Some(c) = &self.credentials {
            parts.push(format!(r#""credentials":{}"#, c.to_json()));
        }
        if let Some(v) = &self.endpoint {
            parts.push(format!(r#""endpoint":"{}""#, escape(v)));
        }
        if let Some(v) = &self.region {
            parts.push(format!(r#""region":"{}""#, escape(v)));
        }
        if let Some(v) = &self.access_id {
            parts.push(format!(r#""access_id":"{}""#, escape(v)));
        }
        if let Some(v) = &self.secret_key {
            parts.push(format!(r#""secret_key":"{}""#, escape(v)));
        }
        if let Some(v) = &self.bucket {
            parts.push(format!(r#""bucket":"{}""#, escape(v)));
        }
        format!("{{{}}}", parts.join(","))
    }
}

#[derive(Debug, Clone, Default)]
pub struct StorageConfig {
    pub endpoint: String,
    pub region: String,
    pub bucket: String,
    pub output_directory: String,
    pub access_id: String,
    pub secret_key: String,
}

impl StorageConfig {
    pub fn to_json(&self) -> String {
        if self.endpoint.is_empty()
            && self.region.is_empty()
            && self.bucket.is_empty()
            && self.access_id.is_empty()
            && self.secret_key.is_empty()
        {
            return if self.output_directory.is_empty() {
                "{}".to_string()
            } else {
                format!(r#"{{"output_directory":"{}"}}"#, escape(&self.output_directory))
            };
        }
        format!(
            r#"{{"endpoint":"{}","region":"{}","bucket":"{}","output_directory":"{}","access_id":"{}","secret_key":"{}"}}"#,
            escape(&self.endpoint),
            escape(&self.region),
            escape(&self.bucket),
            escape(&self.output_directory),
            escape(&self.access_id),
            escape(&self.secret_key)
        )
    }
}

#[derive(Debug, Clone)]
pub struct TranscoderWebhook {
    pub webhook_url: String,
}

impl TranscoderWebhook {
    pub fn to_json(&self) -> String {
        format!(r#"{{"webhook_url":"{}"}}"#, escape(&self.webhook_url))
    }
}

#[derive(Debug, Clone)]
pub struct CreateJobRequest {
    pub source: Source,
    pub outputs: BTreeMap<String, String>,
    pub destination: Option<StorageConfig>,
    pub webhook: Option<TranscoderWebhook>,
}

impl CreateJobRequest {
    pub fn to_json(&self) -> String {
        let mut parts = vec![
            format!(r#""source":{}"#, self.source.to_json()),
            format!(r#""outputs":{}"#, map_to_json(&self.outputs)),
        ];
        if let Some(d) = &self.destination {
            parts.push(format!(r#""destination":{}"#, d.to_json()));
        }
        if let Some(w) = &self.webhook {
            parts.push(format!(r#""webhook":{}"#, w.to_json()));
        }
        format!("{{{}}}", parts.join(","))
    }
}

fn map_to_json(m: &BTreeMap<String, String>) -> String {
    let items: Vec<String> = m
        .iter()
        .map(|(k, v)| format!(r#""{}":{}"#, escape(k), v))
        .collect();
    format!("{{{}}}", items.join(","))
}

#[derive(Debug, Clone, Default)]
pub struct OutputDetail {
    pub name: Option<String>,
    pub path: Option<String>,
    pub status: Option<String>,
    pub duration: Option<String>,
    pub resolution: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct Job {
    pub id: String,
    pub project_id: Option<String>,
    pub status: String,
    pub filename: Option<String>,
    pub download_progress: i64,
    pub upload_progress: i64,
    pub video_codec: Option<String>,
    pub audio_codec: Option<String>,
    pub frame_rate: Option<String>,
    pub bitrate: Option<String>,
    pub color_space: Option<String>,
    pub source_service: Option<String>,
    pub output_formats: Vec<String>,
    pub output_details: Vec<OutputDetail>,
}

impl Job {
    pub fn from_json(s: &str) -> Result<Self, Error> {
        // unwrap envelope if present
        let body = if let Some(idx) = s.find("\"data\"") {
            // extract object after "data":
            extract_object_after(s, idx).unwrap_or_else(|| s.to_string())
        } else {
            s.to_string()
        };
        let id = extract_string(&body, "id").unwrap_or_default();
        let status = extract_string(&body, "status").unwrap_or_default();
        Ok(Job {
            id,
            project_id: extract_string(&body, "project_id"),
            status,
            filename: extract_string(&body, "filename"),
            download_progress: extract_i64(&body, "download_progress").unwrap_or(0),
            upload_progress: extract_i64(&body, "upload_progress").unwrap_or(0),
            video_codec: extract_string(&body, "video_codec"),
            audio_codec: extract_string(&body, "audio_codec"),
            frame_rate: extract_string(&body, "frame_rate"),
            bitrate: extract_string(&body, "bitrate"),
            color_space: extract_string(&body, "color_space"),
            source_service: extract_string(&body, "source_service"),
            output_formats: Vec::new(),
            output_details: Vec::new(),
        })
    }
}

#[derive(Debug)]
pub enum Error {
    Transport(String),
    Api {
        status: u16,
        message: String,
        error: String,
        body: String,
    },
    InvalidConfig(String),
    Parse(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Transport(m) => write!(f, "jetencode: transport: {}", m),
            Error::Api {
                status,
                error,
                message,
                ..
            } => {
                if !error.is_empty() {
                    write!(f, "jetencode: HTTP {}: {}", status, error)
                } else {
                    write!(f, "jetencode: HTTP {}: {}", status, message)
                }
            }
            Error::InvalidConfig(m) => write!(f, "jetencode: {}", m),
            Error::Parse(m) => write!(f, "jetencode: parse: {}", m),
        }
    }
}

impl std::error::Error for Error {}

#[derive(Debug, Clone)]
pub struct Response {
    pub status: u16,
    pub body: String,
}

pub trait Transport: Send + Sync {
    fn request(
        &self,
        method: &str,
        url: &str,
        headers: &[(String, String)],
        body: Option<&str>,
    ) -> Result<Response, Error>;
}

pub struct HttpTransport {
    pub timeout: Duration,
}

impl HttpTransport {
    pub fn new(timeout: Duration) -> Self {
        Self { timeout }
    }
}

impl Transport for HttpTransport {
    fn request(
        &self,
        method: &str,
        url: &str,
        headers: &[(String, String)],
        body: Option<&str>,
    ) -> Result<Response, Error> {
        // only http:// supported
        let url = url.trim_start_matches("http://");
        let (host, path) = match url.find('/') {
            Some(i) => (&url[..i], &url[i..]),
            None => (url, "/"),
        };
        let (host, port) = match host.find(':') {
            Some(i) => (&host[..i], host[i + 1..].parse::<u16>().unwrap_or(80)),
            None => (host, 80),
        };
        let mut stream =
            TcpStream::connect((host, port)).map_err(|e| Error::Transport(e.to_string()))?;
        stream.set_read_timeout(Some(self.timeout)).ok();
        let mut req = format!("{} {} HTTP/1.1\r\nHost: {}\r\n", method, path, host);
        for (k, v) in headers {
            req.push_str(&format!("{}: {}\r\n", k, v));
        }
        if let Some(b) = body {
            req.push_str(&format!("Content-Length: {}\r\n", b.len()));
        }
        req.push_str("Connection: close\r\n\r\n");
        if let Some(b) = body {
            req.push_str(b);
        }
        stream
            .write_all(req.as_bytes())
            .map_err(|e| Error::Transport(e.to_string()))?;
        let mut buf = Vec::new();
        stream
            .read_to_end(&mut buf)
            .map_err(|e| Error::Transport(e.to_string()))?;
        let text = String::from_utf8_lossy(&buf).to_string();
        // parse status
        let status = text
            .lines()
            .next()
            .and_then(|l| l.split_whitespace().nth(1))
            .and_then(|s| s.parse::<u16>().ok())
            .unwrap_or(0);
        // body after blank line
        let body = text.split("\r\n\r\n").nth(1).unwrap_or("").to_string();
        Ok(Response { status, body })
    }
}

pub struct Config {
    pub base_url: String,
    pub project_id: String,
    pub api_key: String,
    pub timeout: Duration,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            base_url: String::new(),
            project_id: String::new(),
            api_key: String::new(),
            timeout: Duration::from_secs(30),
        }
    }
}

pub struct Client {
    base_url: String,
    project_id: String,
    api_key: String,
    transport: Box<dyn Transport>,
}

impl Client {
    pub fn new(cfg: Config) -> Result<Self, Error> {
        if cfg.base_url.trim().is_empty() {
            return Err(Error::InvalidConfig("base_url is required".into()));
        }
        if cfg.project_id.trim().is_empty() {
            return Err(Error::InvalidConfig("project_id is required".into()));
        }
        if cfg.api_key.trim().is_empty() {
            return Err(Error::InvalidConfig("api_key is required".into()));
        }
        Ok(Self {
            base_url: cfg.base_url.trim_end_matches('/').to_string(),
            project_id: cfg.project_id,
            api_key: cfg.api_key,
            transport: Box::new(HttpTransport::new(cfg.timeout)),
        })
    }

    pub fn with_transport(mut self, t: Box<dyn Transport>) -> Self {
        self.transport = t;
        self
    }

    fn request(&self, method: &str, path: &str, body: Option<String>) -> Result<String, Error> {
        let url = format!("{}{}", self.base_url, path);
        let h: Vec<(String, String)> = vec![
            ("Content-Type".to_string(), "application/json".to_string()),
            ("Accept".to_string(), "application/json".to_string()),
            ("X-Project-ID".to_string(), self.project_id.clone()),
            (
                "Authorization".to_string(),
                format!("Bearer {}", self.api_key),
            ),
        ];
        let resp = self.transport.request(method, &url, &h, body.as_deref())?;
        if resp.status < 200 || resp.status >= 300 {
            // try parse message/error
            let message = extract_string(&resp.body, "message").unwrap_or_default();
            let err = extract_string(&resp.body, "error").unwrap_or_default();
            return Err(Error::Api {
                status: resp.status,
                message,
                error: err,
                body: resp.body,
            });
        }
        Ok(resp.body)
    }

    pub fn create_job(&self, req: &CreateJobRequest) -> Result<Job, Error> {
        let body = req.to_json();
        let resp = self.request("POST", "/api/v1/job", Some(body))?;
        Job::from_json(&resp)
    }

    pub fn get_job(&self, job_id: &str) -> Result<Job, Error> {
        let path = format!("/api/v1/job/{}", job_id);
        let resp = self.request("GET", &path, None)?;
        Job::from_json(&resp)
    }

    pub fn cancel_job(&self, job_id: &str) -> Result<(), Error> {
        let path = format!("/api/v1/job/{}/cancel", job_id);
        self.request("POST", &path, None)?;
        Ok(())
    }
}

fn extract_string(s: &str, key: &str) -> Option<String> {
    let needle = format!(r#""{}""#, key);
    let idx = s.find(&needle)?;
    let rest = &s[idx + needle.len()..];
    // find colon
    let colon = rest.find(':')?;
    let rest = rest[colon + 1..].trim_start();
    if let Some(stripped) = rest.strip_prefix('"') {
        // read until unescaped quote
        let mut out = String::new();
        let mut chars = stripped.chars();
        while let Some(c) = chars.next() {
            if c == '"' {
                break;
            }
            if c == '\\' {
                if let Some(next) = chars.next() {
                    out.push(next);
                }
            } else {
                out.push(c);
            }
        }
        Some(out)
    } else {
        None
    }
}

fn extract_i64(s: &str, key: &str) -> Option<i64> {
    let needle = format!(r#""{}""#, key);
    let idx = s.find(&needle)?;
    let rest = &s[idx + needle.len()..];
    let colon = rest.find(':')?;
    let rest = rest[colon + 1..].trim_start();
    // read digits
    let mut digits = String::new();
    for c in rest.chars() {
        if c.is_ascii_digit() || c == '-' {
            digits.push(c);
        } else {
            break;
        }
    }
    digits.parse().ok()
}

fn extract_object_after(s: &str, idx: usize) -> Option<String> {
    // find first '{' after idx
    let sub = &s[idx..];
    let start = sub.find('{')?;
    let mut depth = 0;
    let mut end = None;
    for (i, c) in sub[start..].char_indices() {
        match c {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    end = Some(start + i + 1);
                    break;
                }
            }
            _ => {}
        }
    }
    end.map(|e| sub[start..e].to_string())
}

fn escape(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
}

// helper for examples
pub fn example() {}
