import {
  Client,
  type CreateJobRequest,
} from "../src/index.js";

async function main(): Promise<void> {
  const client = new Client({
    baseURL: "https://your-jetencode-host",
    projectID: "3fa85f64-5717-4562-b3fc-2c963f66afa6",
    apiKey: "your-api-key",
  });

  const request: CreateJobRequest = {
    source: {
      service: "s3",
      path: "platform/videos/movie.mp4",
    },
    outputs: {
      "mp4:720p": {},
    },
  };

  const job = await client.CreateJob(request);

  console.log("Job ID:", job.id);
  console.log("Job status:", job.status);

  // Fetch the job later.
  const currentJob = await client.GetJob(job.id);

  console.log("Current status:", currentJob.status);

  // Uncomment if you want to request cancellation.
  // await client.CancelJob(job.id);
  // console.log("Cancellation requested.");
}

main().catch((error: unknown) => {
  console.error("JetEncode request failed:", error);
});
