package common

import (
	"encoding/json"
	"time"
)

type Response struct {
	Data      interface{} `json:"data"`
	Timestamp int64       `json:"timestamp"`
	Querytime int32       `json:"queryduration"`
}

type ResponseError struct {
	Msg       string `json:"message"`
	Err       error  `json:"error"`
	Timestamp int64  `json:"timestamp"`
	Querytime int32  `json:"queryduration"`
}

func NewResponse(data interface{}) Response {
	return Response{
		data,
		time.Now().UnixNano(),
		-1,
	}
}

func OutResponse(data interface{}) []byte {
	out, _ := json.Marshal(NewResponse(data))
	return out
}

func NewError(msg string, err error) ResponseError {
	return ResponseError{
		msg,
		err,
		time.Now().UnixNano(),
		-1,
	}
}

func OutError(msg string, err error) []byte {
	out, _ := json.Marshal(NewError(msg, err))
	return out
}
