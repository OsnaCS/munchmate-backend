package v1

import (
	"github.com/julienschmidt/httprouter"
)

func PrepareRoutes(r *httprouter.Router) {
	// routes for /canteen
	r.GET("/v1/canteen/nearest", GetNearCanteens)
}
