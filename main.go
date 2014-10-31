package main

import (
	"github.com/go-martini/martini"
	// "munchmate-backend/store"
	"os"
)

func main() {
	// var database store.Database

	// if err != nil {
	// 	return
	// }

	m := martini.Classic()

	// m.Group("/munch", func(r martini.Router) {
	// 	// r.Get("/:id")
	// })

	m.Get("/", func() string {
		return "shit: " + os.Getenv("DATABASE_URL")
	})
	m.Run()
}
