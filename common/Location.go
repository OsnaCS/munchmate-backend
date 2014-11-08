package common

import (
	"strconv"
	"strings"
)

type Location struct {
	Lat float64 `json:"lat"`
	Lng float64 `json:"lng"`
}

func (l *Location) Scan(val interface{}) error {
	// Parse the string obtained from the postgres database, which looks like:
	// (52.270536,8.04554)

	// obtain string from []byte and ignore outer parans
	raw := val.([]byte)
	nums := string(raw[1 : len(raw)-2])

	// parse first number as float
	var err error
	lat := nums[0 : strings.Index(nums, ",")-1]
	l.Lat, err = strconv.ParseFloat(lat, 64)
	if err != nil {
		return err
	}

	// parse second number as float
	lng := nums[strings.Index(nums, ",")+1 : len(nums)-1]
	l.Lng, err = strconv.ParseFloat(lng, 64)
	if err != nil {
		return err
	}

	return nil
}
