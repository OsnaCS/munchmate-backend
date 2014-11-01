package store

import (
	"database/sql"
	_ "github.com/lib/pq"
	"os"
)

type MyDB struct {
	Con *sql.DB
}

func (db *MyDB) Init() error {
	// connect to the postgres database with the database URL retrieved via
	// environment variable
	var conErr error
	db.Con, conErr = sql.Open("postgres", os.Getenv("DATABASE_URL"))

	// check if an error occurred while connecting
	if conErr != nil {
		return conErr
	}

	// make sure that we have tried to connect and check for further errors
	pingErr := db.Con.Ping()
	if pingErr != nil {
		return pingErr
	}

	return nil
}
