export class APIError extends Error {
  readonly statusCode: number;
  readonly message: string;
  readonly errorText: string;
  readonly body: string;

  constructor(
    statusCode: number,
    options: {
      message?: string;
      error?: string;
      body?: string;
    } = {},
  ) {
    const apiMessage = options.message ?? "";
    const errorText = options.error ?? "";

    const description = errorText || apiMessage;
    const errorMessage = description
      ? `jetencode: HTTP ${statusCode}: ${description}`
      : `jetencode: HTTP ${statusCode}`;

    super(errorMessage);

    this.name = "APIError";
    this.statusCode = statusCode;
    this.message = errorMessage;
    this.errorText = errorText;
    this.body = options.body ?? "";

    Object.setPrototypeOf(this, new.target.prototype);
  }
}

export function parseAPIError(
  statusCode: number,
  body: string,
): APIError {
  let message = "";
  let error = "";

  if (body.trim() !== "") {
    try {
      const parsed: unknown = JSON.parse(body);

      if (typeof parsed === "object" && parsed !== null) {
        const record = parsed as Record<string, unknown>;

        if (typeof record.message === "string") {
          message = record.message;
        }

        if (typeof record.error === "string") {
          error = record.error;
        }
      }
    } catch {
      // Preserve the raw response in APIError.body.
    }
  }

  return new APIError(statusCode, {
    message,
    error,
    body,
  });
}