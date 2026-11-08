export interface S3Credentials {
  region: string;
  access_id: string;
  secret_key: string;
  bucket: string;
  endpoint: string;
}

export interface Source {
  service: string;
  url?: string;
  path?: string;
  credentials?: S3Credentials;

  // Optional flat compatibility fields.
  endpoint?: string;
  region?: string;
  access_id?: string;
  secret_key?: string;
  bucket?: string;
}

export interface StorageConfig {
  endpoint: string;
  region: string;
  bucket: string;
  output_directory: string;
  access_id: string;
  secret_key: string;
}

export interface TranscoderWebhook {
  webhook_url: string;
}

export interface CreateJobRequest {
  source: Source;
  outputs: Record<string, unknown>;
  destination?: StorageConfig | { output_directory?: string };
  webhook?: TranscoderWebhook;
}

export interface OutputDetail {
  name: string;
  path?: string;
  status?: string;
  duration?: string;
  resolution?: string;
  variant_count?: number;
  signed_url?: string;
  video_codec?: string;
  audio_codec?: string;
}

export interface Job {
  id: string;
  project_id?: string;
  status: string;
  filename?: string;
  download_progress?: number;
  upload_progress?: number;
  video_codec?: string;
  audio_codec?: string;
  frame_rate?: string;
  bitrate?: string;
  color_space?: string;
  source_service?: string;
  output_formats?: string[];
  output_details?: OutputDetail[];
  created_at?: string;
  processing_seconds?: number;
  duration_seconds?: number;
}
