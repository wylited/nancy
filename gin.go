package main

import (
	"net/http"

	"github.com/gin-gonic/gin"
)

func initGin(port string) *gin.Engine {
	r := gin.Default()
	
	r.GET("/ping", func(c *gin.Context) {
		c.String(http.StatusOK, "pong")
	})

	return r
}
