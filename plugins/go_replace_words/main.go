package main

import (
	"github.com/extism/go-pdk"
	"errors"
	"strings"
)

type Metadata struct {
	Name string `json:"name"`
	Url string `json:"url"`
	Description string `json:"description"`
}

//go:wasmexport metadata
func metadata() int32 {
	// Returns info about the plugin which is included in /api/v4/site
	metadata := Metadata {
		Name: "Test Plugin",
		Url: "https://example.com",
		Description: "Plugin to test Lemmy feature",
	}
	err := pdk.OutputJSON(metadata)
	if err != nil {
		pdk.SetError(err)
		return 1
	}
	return 0
}

//go:wasmexport create_local_post
func create_local_post() int32 {
	params := make(map[string]interface{})
	err := pdk.InputJSON(&params)
	if err != nil {
		pdk.SetError(err)
		return 1
	}

	// Dont allow any posts mentioning Java (these will throw an API error and not be written
	// to the database)
	body := params["body"].(string)
	if strings.Contains(body, "Java") {
		// Throw error to reject post
		pdk.SetError(errors.New("We dont talk about Java"))
		return 1
	}

	// Replace all occurences of "Rust" in post body with "Go"
	params["body"] = strings.Replace(body, "Rust", "Go", -1);

	err = pdk.OutputJSON(params)
	if err != nil {
		pdk.SetError(err)
		return 1
	}
	return 0
}

func main() {}