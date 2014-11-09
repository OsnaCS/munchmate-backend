package v1

import (
	"bytes"
	"crypto/sha1"
	"database/sql"
	"encoding/base64"
	"encoding/hex"
	"encoding/json"
	"errors"
	"github.com/julienschmidt/httprouter"
	"munchmate-backend/common"
	"net/http"
	"net/url"
	"os"
	"strconv"
	"time"
)

const (
	MAX_IMAGE_SIZE        = 1 << 21 // 2 MB
	IMAGE_DIMENSION_BIG   = 512
	IMAGE_QUALITY_BIG     = 85
	IMAGE_DIMENSION_SMALL = 64
	IMAGE_QUALITY_SMALL   = 60
)

type User struct {
	ID        int64  `json:"id"`
	Name      string `json:"name"`
	AvatarURL string `json:"avatarUrl"`
	Thumbnail []byte `json:"thumbnail"`
}

type cloudinaryUploadAnswer struct {
	SecureUrl string `json:"secure_url"`
	PublicID  string `json:"public_id"`
	Version   int64  `json:"version"`
}

func GetUserByID(w http.ResponseWriter, r *http.Request, par httprouter.Params) {
	// try to parse id given by URL
	id, convErr := strconv.ParseInt(par.ByName("id"), 10, 64)
	if convErr != nil {
		w.WriteHeader(http.StatusBadRequest)
		w.Write(common.OutError("Invalid request (id)", convErr))
		return
	}

	// // try to obtain the query value "thumbnail"
	v := r.URL.Query()
	thumb := v.Get("thumbnail")

	var row *sql.Row
	// if the thumbnail is requested -> SELECT it from database
	if thumb == "true" {
		row = common.DB().QueryRow(
			`SELECT id, username, avatar_url, avatar_thumbnail
			 FROM users WHERE id=$1`, id)
	} else {
		// empty binary data as thumbnail
		row = common.DB().QueryRow(
			`SELECT id, username, avatar_url, E''::bytea AS "avatar_thumbnail"
			 FROM users WHERE id=$1`, id)
	}

	// scan data into a user object and a NullString
	var u User
	var url sql.NullString
	queryErr := row.Scan(&u.ID, &u.Name, &url, &u.Thumbnail)

	if queryErr != nil {
		w.WriteHeader(http.StatusInternalServerError)
		w.Write(common.OutError("Query failedxxx!", queryErr))
		return
	}

	// if avatar_url is NULL in DB -> just output empty string
	u.AvatarURL = url.String

	w.Write(common.OutResponse(u))
}

func UpdateUserAvatar(w http.ResponseWriter, r *http.Request, par httprouter.Params) {
	// --- Check Request ------------------------------------------------------
	// check user id
	id, convErr := strconv.ParseInt(par.ByName("id"), 10, 64)
	if convErr != nil {
		w.WriteHeader(http.StatusBadRequest)
		w.Write(common.OutError("Invalid request (id)", convErr))
		return
	}

	// read data of request and try to verify it
	inBuf := new(bytes.Buffer)
	inBuf.ReadFrom(r.Body)
	img := inBuf.Bytes()

	// try to verify data
	if inBuf.Len() <= 2 || inBuf.Len() > MAX_IMAGE_SIZE {
		w.WriteHeader(http.StatusBadRequest)
		w.Write(common.OutError("No image data or image to large (>"+
			strconv.Itoa(MAX_IMAGE_SIZE)+" bytes)!", nil))
		return
	}

	// check for magic bytes: 3 jpeg or first 3 of png
	if !(img[0] == 0xFF && img[1] == 0xD8 && img[2] == 0xFF) &&
		!(img[0] == 0x89 && img[1] == 0x50 && img[2] == 0x4E) {
		w.WriteHeader(http.StatusBadRequest)
		w.Write(common.OutError("Data is not png or jpeg image!", nil))
		return
	}

	// --- Upload image to cloudinary -----------------------------------------
	upRespond, upErr := uploadImage(img)
	if upErr != nil {
		w.WriteHeader(http.StatusServiceUnavailable)
		w.Write(common.OutError("Failed to upload the image!", upErr))
		return
	}

	// --- Download small image and save it into database ---------------------
	thumbnail, thumbErr := getSmallBase64(*upRespond)
	if thumbErr != nil {
		w.WriteHeader(http.StatusServiceUnavailable)
		w.Write(common.OutError("Failed to download thumbnail!", thumbErr))
		return
	}

	// --- Save URL and thumbnial in database ---------------------------------
	res, dbErr := common.DB().Exec(
		`UPDATE users
   		 SET avatar_url=$1, last_action=now(), avatar_thumbnail=$2
 		 WHERE id=$3;`, upRespond.SecureUrl, thumbnail, id)
	affected, affErr := res.RowsAffected()
	if dbErr != nil || (affected != 1 && affErr == nil) {
		w.WriteHeader(http.StatusServiceUnavailable)
		w.Write(common.OutError("Failed save image to DB!", dbErr))
		return
	}

	w.WriteHeader(http.StatusOK)
	w.Write(common.OutResponse("Success"))
}

func uploadImage(img []byte) (*cloudinaryUploadAnswer, error) {
	// prepare cloudinary request
	trans := "c_fill" +
		",w_" + strconv.Itoa(IMAGE_DIMENSION_BIG) +
		",q_" + strconv.Itoa(IMAGE_QUALITY_BIG)
	params := url.Values{
		"timestamp":      {strconv.FormatInt(time.Now().Unix(), 10)},
		"format":         {"jpg"},
		"transformation": {trans},
	}

	// calculate signature
	sigStr, _ := url.QueryUnescape(params.Encode())
	raw := sha1.Sum([]byte(sigStr + os.Getenv("CLOUDINARY_SECRET")))
	sig := hex.EncodeToString(raw[:])

	// add missing parameter
	params.Add("signature", sig)
	params.Add("api_key", os.Getenv("CLOUDINARY_KEY"))
	params.Add("file", "data:image/jpg;base64,"+
		base64.StdEncoding.EncodeToString(img))

	// make post request to cloudinary
	resp, postErr := http.PostForm(
		"http://api.cloudinary.com/v1_1/munchmate/image/upload", params)

	// check if any error occured while sending post request
	if postErr != nil {
		return nil, postErr
	}
	if resp.StatusCode != 200 {
		return nil, errors.New("Uploading image failed (Code: " +
			strconv.Itoa(resp.StatusCode) + ")")
	}

	// read answer of post request and unmarshal into struct
	buf := new(bytes.Buffer)
	buf.ReadFrom(resp.Body)
	var out cloudinaryUploadAnswer
	json.Unmarshal(buf.Bytes(), &out)

	if out.SecureUrl == "" {
		return nil, errors.New("Unexpected answer from cloudinary server! (" +
			string(buf.Bytes()) + ")")
	}
	return &out, nil
}

func getSmallBase64(info cloudinaryUploadAnswer) ([]byte, error) {
	// build url of small image
	trans := "c_fill" +
		",w_" + strconv.Itoa(IMAGE_DIMENSION_SMALL) +
		",q_" + strconv.Itoa(IMAGE_QUALITY_SMALL)
	url := "https://res.cloudinary.com/munchmate/image/upload/" +
		trans + "/v" + strconv.FormatInt(info.Version, 10) +
		"/" + info.PublicID + ".jpg"

	// download image
	resp, getErr := http.Get(url)

	if getErr != nil {
		return nil, getErr
	}
	if resp.StatusCode != 200 {
		return nil, errors.New("Downloading thumbnail failed (Code: " +
			strconv.Itoa(resp.StatusCode) + ")")
	}

	inBuf := new(bytes.Buffer)
	inBuf.ReadFrom(resp.Body)
	img := inBuf.Bytes()
	return img, nil
}
