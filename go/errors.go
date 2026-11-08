package jetencode

import (
	"encoding/json"
	"fmt"
	"net/http"
)

// APIError represents a non-success response from the JetEncode API.
type APIError struct {
	StatusCode int
	Message    string
	ErrorText  string `json:"-"`
	Body       string
}

func (e *APIError) Error() string {
	if e.ErrorText != "" {
		return fmt.Sprintf("jetencode: HTTP %d: %s", e.StatusCode, e.ErrorText)
	}
	if e.Message != "" {
		return fmt.Sprintf("jetencode: HTTP %d: %s", e.StatusCode, e.Message)
	}
	return fmt.Sprintf("jetencode: HTTP %d", e.StatusCode)
}

func parseAPIError(resp *http.Response, body []byte) error {
	var envelope struct {
		Message string `json:"message"`
		Error   string `json:"error"`
	}
	_ = json.Unmarshal(body, &envelope)
	return &APIError{
		StatusCode: resp.StatusCode,
		Message:    envelope.Message,
		ErrorText:  envelope.Error,
		Body:       string(body),
	}
}
