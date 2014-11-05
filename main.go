package main

import (
	"encoding/json"
	"github.com/go-martini/martini"
	"munchmate-backend/store"
	"net/http"
	"strconv"
)

// Initializes the database connection (used as a middleware handler)
func InitDB(res http.ResponseWriter, req *http.Request, db *store.MyDB) {
	// the database tries to connect and ping itself
	err := db.Init()

	// If any error occured -> output error via HTTP status code and text.
	// The output will prevent any other handlers from starting
	if err != nil {
		res.WriteHeader(http.StatusInternalServerError)
		res.Write([]byte("Database connection failed"))
	}
}

func GetUser(db *store.MyDB, params martini.Params) (int, string) {
	// try to convert the given id to an int64
	id, convErr := strconv.ParseInt(params["id"], 10, 64)
	if convErr != nil {
		return 400, "Invalid ID"
	}

	// query the database for the requested user and save result
	us := store.User{ID: id}
	row := db.Con.QueryRow(`SELECT username, has_avatar FROM users 
                            WHERE user_id = $1`, id)
	err := row.Scan(&us.Name, &us.HasAvatar)

	// if any error occured while querying...
	if err != nil {
		return 500, "Query failed: " + err.Error()
	}

	// serialize the user data as json and send out
	out, _ := json.Marshal(us)
	return 200, string(out)
}

func GetNearCanteens(db *store.MyDB, req *http.Request) (int, string) {
	// obtain http-get parameters
	// TODO: Maybe check for sanity
	v := req.URL.Query()

	// execute query
	// TODO: "External" Limit?
	rows, queryErr := db.Con.Query(
		`SELECT canteens.id, city_id, 
				canteens.name, cities.name, location,
				(point($1, $2) <@> location)*1.609344 
					as distance
		 FROM canteens
		 INNER JOIN cities ON cities.id=city_id
		 ORDER BY distance
		 LIMIT 5`, v.Get("lat"), v.Get("lng"))

	// check if any error occured while executing the query
	if queryErr != nil {
		return 500, "Query failed: " + queryErr.Error()
	}
	defer rows.Close()

	// prepare list of canteens
	var canteens []store.Canteen

	// go through all results and add those to the list
	for rows.Next() {
		var c store.Canteen
		rowErr := rows.Scan(&c.ID, &c.CityID, &c.Name, &c.CityName,
			&c.GeoLocation, &c.Distance)
		if rowErr != nil {
			return 500, "Query failed: " + rowErr.Error()
		}
		canteens = append(canteens, c)
	}

	// serialize the user data as json and send out
	out, _ := json.Marshal(canteens)
	return 200, string(out)
}

func main() {
	// create a classic instance of martini
	m := martini.Classic()

	// Make the database service available to all handlers and add middleware
	// handler "InitDB" to initialize the DB connection.
	var database store.MyDB
	m.Map(&database)
	m.Use(InitDB)

	// set routes for user action
	m.Group("/user", func(r martini.Router) {
		r.Get("/:id", GetUser)
	})

	m.Group("/canteen", func(r martini.Router) {
		r.Get("/nearest", GetNearCanteens)
	})

	// set root route (print some message)
	m.Get("/", func(db *store.MyDB) string {
		return "Munchmate Backend :-)"
	})

	// run the martini server
	m.Run()
}
