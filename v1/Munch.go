package v1

import (
	"github.com/julienschmidt/httprouter"
	"munchmate-backend/common"
	"net/http"
	"time"
)

const (
	NUM_UPCOMING_MUNCHES = 3
)

type Munch struct {
	ID          int64     `json:"id"`
	Date        time.Time `json:"date"`
	HostCanteen Canteen   `json:"canteen"`
}

func GetUpcomingMunches(w http.ResponseWriter, r *http.Request, par httprouter.Params) {
	// execute query
	rows, queryErr := common.DB().Query(
		`SELECT munches.id, munches.date, canteens.id, canteens.city_id, 
			    canteens.name, cities.name, canteens.location
		 FROM munches
		 INNER JOIN canteens ON canteens.id=munches.canteen
		 INNER JOIN cities ON cities.id=canteens.city_id
		 WHERE date > now()
		 ORDER BY date ASC
		 LIMIT $1`, NUM_UPCOMING_MUNCHES)

	// check if any error occured while executing the query
	if queryErr != nil {
		w.WriteHeader(http.StatusInternalServerError)
		w.Write(common.OutError("Query failed!", queryErr))
		return
	}
	defer rows.Close()

	// prepare list of canteens
	var munches []Munch

	// go through all results and add those to the list
	for rows.Next() {
		var m Munch
		rowErr := rows.Scan(&m.ID, &m.Date, &m.HostCanteen.ID, &m.HostCanteen.CityID,
			&m.HostCanteen.Name, &m.HostCanteen.CityName,
			&m.HostCanteen.GeoLocation)
		if rowErr != nil {
			w.WriteHeader(http.StatusInternalServerError)
			w.Write(common.OutError("Scanning row failed!", queryErr))
			return
		}
		munches = append(munches, m)
	}

	// serialize the user data as json and send out
	w.Write(common.OutResponse(munches))
}
