package store

import (
	"database/sql"
	_ "github.com/lib/pq"
	"os"
)

type Database struct {
	dbCon *sql.DB
}

func (db *Database) Init() error {
	// connect to the postgres database with the database URL retrieved via
	// environment variable
	var conErr error
	db.dbCon, conErr = sql.Open("postgres", os.Getenv("DATABASE_URL"))

	// check if an error occurred while connecting
	if conErr != nil {
		return conErr
	}

	// make sure that we have tried to connect and check for further errors
	pingErr := db.dbCon.Ping()
	if pingErr != nil {
		return pingErr
	}

	return nil
}
