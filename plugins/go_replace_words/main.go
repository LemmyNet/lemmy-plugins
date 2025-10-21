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
		Name: "Replace Words",
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
//go:wasmexport local_post_before_create
func local_post_before_create() int32 {
	// Load user parameters into a map, to make sure we return all the same fields later
	// and dont drop anything
	params := make(map[string]interface{})
	err := pdk.InputJSON(&params)
	if err != nil {
		pdk.SetError(err)
		return 1
	}

	// Dont allow any posts mentioning Java in title
	// (these will throw an API error and not be written to the database)
	name := params["name"].(string)
	if strings.Contains(name, "Java") {
		// Throw error to reject post
		pdk.SetError(errors.New("We dont talk about Java"))
		return 1
	}

	// Replace all occurences of "Rust" in post title with "Go"
	params["name"] = strings.Replace(name, "Rust", "Go", -1);

	err = pdk.OutputJSON(params)
	if err != nil {
		pdk.SetError(err)
		return 1
	}
	return 0
}
