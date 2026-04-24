package main

import (
	"log"
	"net/http"
)

func main() {
	// Serve static files
	fs := http.FileServer(http.Dir("./static"))
	http.Handle("/", fs)

	port := ":8080"
	log.Printf("🔒 DarkWeb3 Breach Checker starting on http://localhost%s", port)
	log.Printf("Privacy-first data breach monitoring system")
	if err := http.ListenAndServe(port, nil); err != nil {
		log.Fatal(err)
	}
}
