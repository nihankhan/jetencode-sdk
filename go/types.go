package jetencode

import "encoding/json"

// S3Credentials holds credentials for an S3-compatible storage endpoint.
type S3Credentials struct {
	Region    string `json:"region"`
	AccessID  string `json:"access_id"`
	SecretKey string `json:"secret_key"`
	Bucket    string `json:"bucket"`
	Endpoint  string `json:"endpoint"`
}

// Source describes the input of a transcoding job.
type Source struct {
	Service     string         `json:"service"`
	URL         string         `json:"url,omitempty"`
	Path        string         `json:"path,omitempty"`
	Credentials *S3Credentials `json:"credentials,omitempty"`
	// Flat compatibility fields (optional).
	Endpoint  string `json:"endpoint,omitempty"`
	Region    string `json:"region,omitempty"`
	AccessID  string `json:"access_id,omitempty"`
	SecretKey string `json:"secret_key,omitempty"`
	Bucket    string `json:"bucket,omitempty"`
}

// StorageConfig describes where output files are written.
type StorageConfig struct {
	Endpoint        string `json:"endpoint"`
	Region          string `json:"region"`
	Bucket          string `json:"bucket"`
	OutputDirectory string `json:"output_directory"`
	AccessID        string `json:"access_id"`
	SecretKey       string `json:"secret_key"`
}

// MarshalJSON keeps the legacy inline shape, but default storage never emits
// empty credential fields. A directory-only configuration overrides the prefix.
func (s StorageConfig) MarshalJSON() ([]byte, error) {
	type inline StorageConfig
	if s.Endpoint != "" || s.Region != "" || s.Bucket != "" || s.AccessID != "" || s.SecretKey != "" {
		return json.Marshal(inline(s))
	}
	return json.Marshal(struct {
		OutputDirectory string `json:"output_directory,omitempty"`
	}{s.OutputDirectory})
}

// TranscoderWebhook is a per-job webhook override.
type TranscoderWebhook struct {
	WebhookURL string `json:"webhook_url"`
}

// CreateJobRequest is the body sent to POST /api/v1/job.
type CreateJobRequest struct {
	Source      Source             `json:"source"`
	Outputs     map[string]any     `json:"outputs"`
	Destination StorageConfig      `json:"destination,omitempty"`
	Webhook     *TranscoderWebhook `json:"webhook,omitempty"`
}

func (r CreateJobRequest) MarshalJSON() ([]byte, error) {
	var destination *StorageConfig
	if r.Destination != (StorageConfig{}) {
		destination = &r.Destination
	}
	return json.Marshal(struct {
		Source      Source             `json:"source"`
		Outputs     map[string]any     `json:"outputs"`
		Destination *StorageConfig     `json:"destination,omitempty"`
		Webhook     *TranscoderWebhook `json:"webhook,omitempty"`
	}{r.Source, r.Outputs, destination, r.Webhook})
}

// Job is the response returned by job endpoints.
type Job struct {
	ID                string         `json:"id"`
	ProjectID         string         `json:"project_id,omitempty"`
	Status            string         `json:"status"`
	Filename          string         `json:"filename,omitempty"`
	DownloadProgress  int            `json:"download_progress,omitempty"`
	UploadProgress    int            `json:"upload_progress,omitempty"`
	VideoCodec        string         `json:"video_codec,omitempty"`
	AudioCodec        string         `json:"audio_codec,omitempty"`
	FrameRate         string         `json:"frame_rate,omitempty"`
	Bitrate           string         `json:"bitrate,omitempty"`
	ColorSpace        string         `json:"color_space,omitempty"`
	SourceService     string         `json:"source_service,omitempty"`
	OutputFormats     []string       `json:"output_formats,omitempty"`
	OutputDetails     []OutputDetail `json:"output_details,omitempty"`
	CreatedAt         string         `json:"created_at,omitempty"`
	ProcessingSeconds int64          `json:"processing_seconds,omitempty"`
	DurationSeconds   float64        `json:"duration_seconds,omitempty"`
}

// OutputDetail holds per-output display data.
type OutputDetail struct {
	Name         string `json:"name"`
	Path         string `json:"path,omitempty"`
	Status       string `json:"status,omitempty"`
	Duration     string `json:"duration,omitempty"`
	Resolution   string `json:"resolution,omitempty"`
	VariantCount int    `json:"variant_count,omitempty"`
	SignedURL    string `json:"signed_url,omitempty"`
	VideoCodec   string `json:"video_codec,omitempty"`
	AudioCodec   string `json:"audio_codec,omitempty"`
}

// envelope is the standard API response wrapper.
type envelope struct {
	Data    json.RawMessage `json:"data"`
	Message string          `json:"message"`
	Error   string          `json:"error"`
}
