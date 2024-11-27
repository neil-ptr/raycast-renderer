package main

import (
	"encoding/json"
	"fmt"
	"net/http"
)

// Data structure for the REST API response
type ApiResponse struct {
	Message string `json:"message"`
}

func main() {
	// Serve static files from the "static" directory
	fs := http.FileServer(http.Dir("./static"))
	http.Handle("/", fs)

	// Define the REST API endpoint
	http.HandleFunc("/api/hello", func(w http.ResponseWriter, r *http.Request) {
		// Set response headers
		w.Header().Set("Content-Type", "application/json")

		// Create response object
		response := ApiResponse{
			Message: "Hello from the Go REST API!",
		}

		// Write JSON response
		json.NewEncoder(w).Encode(response)
	})

	// Start the server
	port := 8080
	fmt.Printf("Server running at http://localhost:%d\n", port)
	if err := http.ListenAndServe(fmt.Sprintf(":%d", port), nil); err != nil {
		panic(err)
	}
}
