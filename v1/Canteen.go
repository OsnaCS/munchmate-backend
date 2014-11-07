package v1

import (
	"github.com/julienschmidt/httprouter"
	"munchmate-backend/common"
	"net/http"
)

type Canteen struct {
	ID          int32
	CityID      int32
	Name        string
	CityName    string
	GeoLocation common.Location
	Distance    float64
}

func GetNearCanteens(w http.ResponseWriter, r *http.Request, par httprouter.Params) {
	// obtain http-get parameters
	// TODO: Maybe check for sanity
	v := r.URL.Query()

	// execute query
	// TODO: "External" Limit?
	rows, queryErr := common.DB().Query(
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
		w.WriteHeader(http.StatusInternalServerError)
		w.Write(common.OutError("Query failed!", queryErr))
		return
	}
	defer rows.Close()

	// prepare list of canteens
	var canteens []Canteen

	// go through all results and add those to the list
	for rows.Next() {
		var c Canteen
		rowErr := rows.Scan(&c.ID, &c.CityID, &c.Name, &c.CityName,
			&c.GeoLocation, &c.Distance)
		if rowErr != nil {
			w.WriteHeader(http.StatusInternalServerError)
			w.Write(common.OutError("Scanning row failed!", queryErr))
			return
		}
		canteens = append(canteens, c)
	}

	// serialize the user data as json and send out
	w.Write(common.OutResponse(canteens))
}
