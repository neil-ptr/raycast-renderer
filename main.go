package main

import (
	"encoding/json"
	"fmt"
	"net/http"
)

type ApiResponse struct {
	Message string `json:"message"`
}

func main() {
	fs := http.FileServer(http.Dir("./static"))
	http.Handle("/", fs)

	http.HandleFunc("/api/hello", func(w http.ResponseWriter, r *http.Request) {

		w.Header().Set("Content-Type", "application/json")

		response := ApiResponse{
			Message: "Hello from the Go REST API!",
		}

		json.NewEncoder(w).Encode(response)
	})

	port := 8080
	fmt.Printf("Server running at http://localhost:%d\n", port)
	if err := http.ListenAndServe(fmt.Sprintf(":%d", port), nil); err != nil {
		panic(err)
	}
}
