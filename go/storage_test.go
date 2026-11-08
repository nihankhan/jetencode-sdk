package jetencode

import (
	"encoding/json"
	"testing"
)

func TestProjectDefaultSerialization(t *testing.T) {
	r := CreateJobRequest{Source: Source{Service: "s3", Path: "platform/videos/movie.mp4"}, Outputs: map[string]any{"mp4:720p": map[string]any{}}}
	for _, tc := range []struct{ directory, want string }{
		{"", `{"source":{"service":"s3","path":"platform/videos/movie.mp4"},"outputs":{"mp4:720p":{}}}`},
		{"custom/", `{"source":{"service":"s3","path":"platform/videos/movie.mp4"},"outputs":{"mp4:720p":{}},"destination":{"output_directory":"custom/"}}`},
	} {
		r.Destination = StorageConfig{OutputDirectory: tc.directory}
		raw, err := json.Marshal(r)
		if err != nil || string(raw) != tc.want {
			t.Fatalf("got %s (%v), want %s", raw, err, tc.want)
		}
	}
	r.Source.Credentials = &S3Credentials{Region: "auto", AccessID: "legacy-id", SecretKey: "legacy-secret", Bucket: "legacy", Endpoint: "https://s3.test"}
	r.Destination = StorageConfig{Endpoint: "https://output.test", Region: "auto", Bucket: "output", OutputDirectory: "out/", AccessID: "output-id", SecretKey: "output-secret"}
	raw, _ := json.Marshal(r)
	want := `{"source":{"service":"s3","path":"platform/videos/movie.mp4","credentials":{"region":"auto","access_id":"legacy-id","secret_key":"legacy-secret","bucket":"legacy","endpoint":"https://s3.test"}},"outputs":{"mp4:720p":{}},"destination":{"endpoint":"https://output.test","region":"auto","bucket":"output","output_directory":"out/","access_id":"output-id","secret_key":"output-secret"}}`
	if string(raw) != want {
		t.Fatalf("legacy JSON changed: %s", raw)
	}
}
