package jetencode

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"net/url"
	"strings"
	"time"
)

// Config configures a Client.
type Config struct {
	BaseURL    string
	ProjectID  string
	APIKey     string
	HTTPClient *http.Client
	Timeout    time.Duration
}

// Client is a JetEncode API client.
type Client struct {
	baseURL   string
	projectID string
	apiKey    string
	http      *http.Client
}

// NewClient creates a new Client. BaseURL must not be empty.
func NewClient(cfg Config) (*Client, error) {
	if strings.TrimSpace(cfg.BaseURL) == "" {
		return nil, fmt.Errorf("jetencode: base URL is required")
	}
	if strings.TrimSpace(cfg.ProjectID) == "" {
		return nil, fmt.Errorf("jetencode: project ID is required")
	}
	if strings.TrimSpace(cfg.APIKey) == "" {
		return nil, fmt.Errorf("jetencode: API key is required")
	}
	hc := cfg.HTTPClient
	if hc == nil {
		timeout := cfg.Timeout
		if timeout == 0 {
			timeout = 30 * time.Second
		}
		hc = &http.Client{Timeout: timeout}
	}
	return &Client{
		baseURL:   strings.TrimRight(cfg.BaseURL, "/"),
		projectID: cfg.ProjectID,
		apiKey:    cfg.APIKey,
		http:      hc,
	}, nil
}

func (c *Client) do(ctx context.Context, method, path string, body any) ([]byte, error) {
	var reqBody io.Reader
	if body != nil {
		b, err := json.Marshal(body)
		if err != nil {
			return nil, err
		}
		reqBody = bytes.NewReader(b)
	}
	req, err := http.NewRequestWithContext(ctx, method, c.baseURL+path, reqBody)
	if err != nil {
		return nil, err
	}
	req.Header.Set("Content-Type", "application/json")
	req.Header.Set("Accept", "application/json")
	req.Header.Set("X-Project-ID", c.projectID)
	req.Header.Set("Authorization", "Bearer "+c.apiKey)

	resp, err := c.http.Do(req)
	if err != nil {
		return nil, err
	}
	defer resp.Body.Close()
	data, err := io.ReadAll(resp.Body)
	if err != nil {
		return nil, err
	}
	if resp.StatusCode < 200 || resp.StatusCode >= 300 {
		return nil, parseAPIError(resp, data)
	}
	return data, nil
}

// CreateJob submits a new transcoding job.
func (c *Client) CreateJob(ctx context.Context, req CreateJobRequest) (*Job, error) {
	data, err := c.do(ctx, http.MethodPost, "/api/v1/job", req)
	if err != nil {
		return nil, err
	}
	var job Job
	// Response may be either the job directly or wrapped in the envelope.
	if err := unmarshalJob(data, &job); err != nil {
		return nil, err
	}
	return &job, nil
}

// GetJob fetches a job by ID.
func (c *Client) GetJob(ctx context.Context, jobID string) (*Job, error) {
	data, err := c.do(ctx, http.MethodGet, "/api/v1/job/"+url.PathEscape(jobID), nil)
	if err != nil {
		return nil, err
	}
	var job Job
	if err := unmarshalJob(data, &job); err != nil {
		return nil, err
	}
	return &job, nil
}

// CancelJob requests cancellation of a job.
func (c *Client) CancelJob(ctx context.Context, jobID string) error {
	_, err := c.do(ctx, http.MethodPost, "/api/v1/job/"+url.PathEscape(jobID)+"/cancel", nil)
	return err
}

func unmarshalJob(data []byte, job *Job) error {
	// Try envelope first.
	var env envelope
	if err := json.Unmarshal(data, &env); err == nil && len(env.Data) > 0 && string(env.Data) != "null" {
		if err := json.Unmarshal(env.Data, job); err == nil {
			return nil
		}
	}
	// Fall back to direct job object.
	return json.Unmarshal(data, job)
}
