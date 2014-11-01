package main

import (
	"github.com/go-martini/martini"
	"munchmate-backend/store"
	"net/http"
)

func InitDB(res http.ResponseWriter, req *http.Request, db *store.Database) {
	err := db.Init()

	if err != nil {
		res.WriteHeader(http.StatusInternalServerError)
		res.Write([]byte("Database connection failed"))
	}
}

func main() {
	var database store.Database

	m := martini.Classic()
	m.Map(&database)

	m.Use(InitDB)

	// m.Group("/munch", func(r martini.Router) {
	// 	// r.Get("/:id")
	// })

	m.Get("/", func(db *store.Database) string {
		return "Yo works yo"
	})
	m.Run()
}
