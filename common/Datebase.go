package common

import (
	"database/sql"
	_ "github.com/lib/pq"
	"os"
)

var database *sql.DB

func DB() *sql.DB {
	return database
}

func ConnectDB() error {
	// connect to the postgres database with the database URL retrieved via
	// environment variable
	var conErr error
	database, conErr = sql.Open("postgres", os.Getenv("DATABASE_URL"))

	// check if an error occurred while connecting
	if conErr != nil {
		return conErr
	}

	// make sure that we have tried to connect and check for further errors
	pingErr := database.Ping()
	if pingErr != nil {
		return pingErr
	}

	return nil
}

func CheckDB() error {
	return DB().Ping()
}
