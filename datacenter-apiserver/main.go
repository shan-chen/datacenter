package main

import (
	"github.com/datacenter-apiserver/business"
	"github.com/gin-gonic/gin"
)

var router *gin.Engine

func main() {
	business.InitSDK()
	router = gin.Default()
	initRouter()
	router.Run()
}

func initRouter() {
	group := router.Group("/data")
	group.GET("/search", business.SearchByKeyWord)
	group.POST("/callback", business.SearchCallBack)
	group.GET("/detail", business.GetDataDetail)
}
