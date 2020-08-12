package model

type MetaData struct {
	IDs      []string `json:"ids"`
	KeyWords string   `json:"keywords"`
	Owner    string   `json:"owner"`
}

type Data struct {
	Owner       string   `json:"owner"`
	Description string   `json:"description"`
	Keys        []string `json:"keys"`
	ID          string   `json:"id"`
}

type QueryResult struct {
	Payload  string `json:"payload"`
	Bookmark string `json:"bookmark"`
}

type SGXResult struct {
	A []Article
}

type Article struct {
	Id    string
	Score float32
}

type QueryLog struct {
	Owner     string `json:"owner"`
	Payload   string `json:"payload"`
	TimeStamp string `json:"timestamp"`
}
