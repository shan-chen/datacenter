package model

type MetaData struct {
	IDs     []string `json:"ids" binding:"required"`
	Owner   string   `json:"owner" binding:"required"`
	KeyWord string   `json:"keyword" binding:"required"`
}
