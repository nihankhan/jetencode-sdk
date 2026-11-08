package main

import (
	"context"
	"fmt"

	jetencode "github.com/GotipathTeam/jetencode/sdk/go"
)

func main() {
	client, _ := jetencode.NewClient(jetencode.Config{
		BaseURL:   "https://your-jetencode-host",
		ProjectID: "3fa85f64-5717-4562-b3fc-2c963f66afa6",
		APIKey:    "your-api-key",
	})
	job, err := client.CreateJob(context.Background(), jetencode.CreateJobRequest{
		Source:  jetencode.Source{Service: "s3", Path: "platform/videos/movie.mp4"},
		Outputs: map[string]any{"mp4:720p": map[string]any{}},
	})
	if err != nil {
		panic(err)
	}
	fmt.Println(job.ID, job.Status)
}
