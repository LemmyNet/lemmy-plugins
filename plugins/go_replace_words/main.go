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

// Returns info about the plugin which gets included in /api/v4/site
//go:wasmexport metadata
func metadata() int32 {
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

// This hook gets called when a local user creates a new post
//go:wasmexport create_local_post
func create_local_post() int32 {
	// Load user parameters into a map, to make sure we return all the same fields later
	// and dont drop anything
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
