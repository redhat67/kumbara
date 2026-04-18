package main

import (
	"log"
	"net/http"
)

func main() {
	// Static dosyaları serve et
	fs := http.FileServer(http.Dir("./static"))
	http.Handle("/", fs)

	port := ":8080"
	log.Printf("Stellar Kumbara sistemi başlatılıyor: http://localhost%s", port)
	if err := http.ListenAndServe(port, nil); err != nil {
		log.Fatal(err)
	}
}
