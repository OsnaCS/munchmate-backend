package v1

import (
	"github.com/julienschmidt/httprouter"
)

func PrepareRoutes(r *httprouter.Router) {
	// routes for /canteen
	r.GET("/v1/canteen/nearest", GetNearCanteens)
	r.GET("/v1/canteen/id/:id", GetCanteenByID)

	// routes for /user
	r.GET("/v1/user/:id", GetUserByID)
	r.PUT("/v1/user/:id/avatar", UpdateUserAvatar)
}
