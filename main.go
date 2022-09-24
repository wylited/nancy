package main

import (
	"github.com/gin-gonic/gin"
	"go.mongodb.org/mongo-driver/mongo"
)

// To use for the account types
//
// type Account string
// const (
// 	checking Account = "checking"
// 	savings  Account = "savings"
// 	card     Account = "card"
// 	invest   Account = "investment"
// 	mortage  Account = "mortage"
// 	other    Account = "other"
// )

// Make a config file for these two.
// possibly using viper.
const uri = "x"
const port = "8080"

type Nancy struct {
	uri    string
	port   string
	gin    *gin.Engine
	client *mongo.Client
}

func nancyInit(uri string, port string) *Nancy {
	return &Nancy{
		uri:    uri,
		port:   port,
		gin:    initGin(port),
		client: initMongo(uri),
	}
}

func main() {
	nancy := nancyInit(uri, port)
	initApi(nancy.gin, nancy.client)
	nancy.gin.Run(nancy.port)
}
