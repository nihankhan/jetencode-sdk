import { APIError, parseAPIError } from "./errors.js";
import type {
  CreateJobRequest,
  Job,
} from "./types.js";

export interface Config {
  baseURL: string;
  projectID: string;
  apiKey: string;
  timeoutMs?: number;
  fetch?: typeof globalThis.fetch;
}

export class Client {
  private readonly baseURL: string;
  private readonly projectID: string;
  private readonly apiKey: string;
  private readonly timeoutMs: number;
  private readonly fetchFn: typeof globalThis.fetch;

  constructor(config: Config) {
    if (!config.baseURL?.trim()) {
      throw new Error("jetencode: base URL is required");
    }

    if (!config.projectID?.trim()) {
      throw new Error("jetencode: project ID is required");
    }

    if (!config.apiKey?.trim()) {
      throw new Error("jetencode: API key is required");
    }

    const timeoutMs = config.timeoutMs ?? 30_000;

    if (!Number.isFinite(timeoutMs) || timeoutMs <= 0) {
      throw new Error(
        "jetencode: timeout must be a positive number of milliseconds",
      );
    }

    this.baseURL = config.baseURL.replace(/\/+$/, "");
    this.projectID = config.projectID;
    this.apiKey = config.apiKey;
    this.timeoutMs = timeoutMs;
    this.fetchFn = config.fetch ?? globalThis.fetch;

    if (typeof this.fetchFn !== "function") {
      throw new Error(
        "jetencode: fetch is unavailable; use Node.js 18 or newer",
      );
    }
  }

  /**
   * CreateJob submits a new transcoding job.
   */
  async CreateJob(request: CreateJobRequest): Promise<Job> {
    const data = await this.do(
      "POST",
      "/api/v1/job",
      request,
    );

    return this.unmarshalJob(data);
  }

  /**
   * GetJob fetches a job by ID.
   */
  async GetJob(jobID: string): Promise<Job> {
    this.validateJobID(jobID);

    const data = await this.do(
      "GET",
      `/api/v1/job/${encodeURIComponent(jobID)}`,
    );

    return this.unmarshalJob(data);
  }

  /**
   * CancelJob requests cancellation of a job.
   */
  async CancelJob(jobID: string): Promise<void> {
    this.validateJobID(jobID);

    await this.do(
      "POST",
      `/api/v1/job/${encodeURIComponent(jobID)}/cancel`,
    );
  }

  private validateJobID(jobID: string): void {
    if (!jobID?.trim()) {
      throw new Error("jetencode: job ID is required");
    }
  }

  private async do(
    method: string,
    path: string,
    body?: unknown,
  ): Promise<unknown> {
    const controller = new AbortController();
    const timeout = setTimeout(
      () => controller.abort(),
      this.timeoutMs,
    );

    try {
      const headers: Record<string, string> = {
        Accept: "application/json",
        "Content-Type": "application/json",
        "X-Project-ID": this.projectID,
        Authorization: `Bearer ${this.apiKey}`,
      };

      const response = await this.fetchFn(
        `${this.baseURL}${path}`,
        {
          method,
          headers,
          signal: controller.signal,
          ...(body === undefined
            ? {}
            : { body: JSON.stringify(body) }),
        },
      );

      const responseBody = await response.text();

      if (!response.ok) {
        throw parseAPIError(
          response.status,
          responseBody,
        );
      }

      if (!responseBody.trim()) {
        return undefined;
      }

      try {
        return JSON.parse(responseBody) as unknown;
      } catch {
        throw new Error(
          "jetencode: server returned invalid JSON",
        );
      }
    } catch (error: unknown) {
      if (error instanceof APIError) {
        throw error;
      }

      if (
        error instanceof Error &&
        error.name === "AbortError"
      ) {
        throw new Error(
          `jetencode: request timed out after ${this.timeoutMs}ms`,
          { cause: error },
        );
      }

      throw error;
    } finally {
      clearTimeout(timeout);
    }
  }

  private unmarshalJob(response: unknown): Job {
    let candidate = response;

    // Accept either { "data": { ...job } } or a direct job object.
    if (
      typeof response === "object" &&
      response !== null &&
      "data" in response
    ) {
      const envelope = response as {
        data?: unknown;
      };

      if (
        envelope.data !== undefined &&
        envelope.data !== null
      ) {
        candidate = envelope.data;
      }
    }

    if (
      typeof candidate !== "object" ||
      candidate === null ||
      Array.isArray(candidate)
    ) {
      throw new Error(
        "jetencode: invalid job response",
      );
    }

    const job = candidate as Partial<Job>;

    if (typeof job.id !== "string" || !job.id) {
      throw new Error(
        "jetencode: job response is missing its ID",
      );
    }

    if (typeof job.status !== "string") {
      throw new Error(
        "jetencode: job response is missing its status",
      );
    }

    return job as Job;
  }
}