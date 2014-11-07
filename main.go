package main

import (
	"fmt"
	"github.com/julienschmidt/httprouter"
	"munchmate-backend/common"
	"munchmate-backend/v1"
	"net/http"
	"os"
)

type extResponseWriter struct {
	status int
	http.ResponseWriter
}

func (w *extResponseWriter) WriteHeader(code int) {
	w.status = code
	w.ResponseWriter.WriteHeader(code)
}

func MainHandler(w http.ResponseWriter, r *http.Request, router *httprouter.Router) {
	// Check if database connection is still alive
	dbErr := common.CheckDB()
	if dbErr != nil {
		w.WriteHeader(http.StatusServiceUnavailable)
		w.Write(common.OutError("Database connection failed!", dbErr))
		return
	}

	// lookup handler for URL
	handler, param, _ := router.Lookup(r.Method, r.URL.Path)
	if handler == nil {
		w.WriteHeader(http.StatusNotFound)
		w.Write(common.OutError("Invalid URL!", nil))
		return
	}

	// call the handler
	handler(w, r, param)

}

func Root(w http.ResponseWriter, r *http.Request, par httprouter.Params) {
	w.Write([]byte("Munchmate API"))
}

func main() {
	fmt.Println(">>>>> Listening on port " + os.Getenv("PORT") + " <<<<<")

	// Connect to database
	dbErr := common.ConnectDB()
	if dbErr != nil {
		fmt.Println("Critical Error: Database connection failed!")
		fmt.Println(dbErr.Error())
	}

	// Create a new smart router
	router := &httprouter.Router{
		RedirectTrailingSlash: false,
		RedirectFixedPath:     false,
	}

	// setup routes (let each version set routes itself)
	router.GET("/", Root)
	v1.PrepareRoutes(router)

	// handle all requests with this lambda
	http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
		// create extended response writer to save status code
		rw := extResponseWriter{200, w}

		// call the main handler which does the router dispatch
		MainHandler(&rw, r, router)

		// print a log message
		fmt.Println(r.Method, r.URL.Path, " -> [",
			rw.status, http.StatusText(rw.status), "]")
	})

	// actually start the server on the port given by $PORT
	http.ListenAndServe(":"+os.Getenv("PORT"), nil)
}
