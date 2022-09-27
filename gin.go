package main

import (
	"net/http"

	"github.com/gin-gonic/gin"
	"go.mongodb.org/mongo-driver/mongo"
)

func initGin(port string) *gin.Engine {
	r := gin.Default()

	r.GET("/ping", func(c *gin.Context) {
		c.JSON(http.StatusOK, gin.H{
			"message": "pong",
		})
	})

	return r
}

func initApi(r *gin.Engine, m *mongo.Client) {
	// List all accounts in MongoDB

	//api/
	//accounts/
	//lsAccounts/
}
