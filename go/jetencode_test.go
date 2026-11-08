package jetencode

import (
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"
)

func TestCreateJob(t *testing.T) {
	srv := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/api/v1/job" || r.Method != http.MethodPost {
			t.Fatalf("unexpected %s %s", r.Method, r.URL.Path)
		}
		if r.Header.Get("X-Project-ID") != "proj" {
			t.Fatal("missing project header")
		}
		if r.Header.Get("Authorization") != "Bearer key" {
			t.Fatal("missing auth header")
		}
		w.WriteHeader(http.StatusCreated)
		_ = json.NewEncoder(w).Encode(map[string]any{
			"message": "created",
			"data": map[string]any{
				"id":     "job-1",
				"status": "queued",
			},
		})
	}))
	defer srv.Close()

	client, err := NewClient(Config{BaseURL: srv.URL, ProjectID: "proj", APIKey: "key"})
	if err != nil {
		t.Fatal(err)
	}
	job, err := client.CreateJob(context.Background(), CreateJobRequest{
		Source:  Source{Service: "http", URL: "https://example.com/v.mp4"},
		Outputs: map[string]any{"mp4:720p": map[string]any{}},
	})
	if err != nil {
		t.Fatal(err)
	}
	if job.ID != "job-1" || job.Status != "queued" {
		t.Fatalf("got %+v", job)
	}
}

func TestGetJobDirect(t *testing.T) {
	srv := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusOK)
		_ = json.NewEncoder(w).Encode(map[string]any{
			"id":     "job-2",
			"status": "completed",
		})
	}))
	defer srv.Close()

	client, _ := NewClient(Config{BaseURL: srv.URL, ProjectID: "proj", APIKey: "key"})
	job, err := client.GetJob(context.Background(), "job-2")
	if err != nil {
		t.Fatal(err)
	}
	if job.ID != "job-2" || job.Status != "completed" {
		t.Fatalf("got %+v", job)
	}
}

func TestCancelJob(t *testing.T) {
	srv := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.Method != http.MethodPost || r.URL.Path != "/api/v1/job/job-3/cancel" {
			t.Fatalf("unexpected %s %s", r.Method, r.URL.Path)
		}
		w.WriteHeader(http.StatusAccepted)
		_ = json.NewEncoder(w).Encode(map[string]any{"message": "Job cancelled successfully"})
	}))
	defer srv.Close()

	client, _ := NewClient(Config{BaseURL: srv.URL, ProjectID: "proj", APIKey: "key"})
	if err := client.CancelJob(context.Background(), "job-3"); err != nil {
		t.Fatal(err)
	}
}

func TestAPIError(t *testing.T) {
	srv := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusUnauthorized)
		_ = json.NewEncoder(w).Encode(map[string]any{
			"message": "Unauthorized",
			"error":   "invalid project id or api key",
		})
	}))
	defer srv.Close()

	client, _ := NewClient(Config{BaseURL: srv.URL, ProjectID: "proj", APIKey: "key"})
	_, err := client.GetJob(context.Background(), "job-4")
	if err == nil {
		t.Fatal("expected error")
	}
	ae, ok := err.(*APIError)
	if !ok {
		t.Fatalf("expected *APIError, got %T", err)
	}
	if ae.StatusCode != http.StatusUnauthorized || ae.ErrorText != "invalid project id or api key" {
		t.Fatalf("got %+v", ae)
	}
}
