package main

import (
	"context"

	"go.mongodb.org/mongo-driver/mongo"
	"go.mongodb.org/mongo-driver/mongo/options"

	"github.com/gin-gonic/gin"
)

type Account string

const (
	checking Account = "checking"
	savings  Account = "savings"
	credit   Account = "credit card"
	invest   Account = "investment"
	mortage  Account = "mortage"
	other    Account = "other"
)

func main() {
	uri := "mongodb://localhost:27017" //!TODO Make this config editable.

	//Initialize gin connection
	r := gin.Default()

	r.Run(":8080") //!TODO Make the port config client adjustable.

	// Initialize mongo connection
	client, err := mongo.Connect(context.TODO(), options.Client().ApplyURI(uri))

	if err != nil {
		panic(err) //!TODO throw more sustainable errors
	}

	na := client.Database("HSBC") // current example
}

// func ginInit() {
// 	r := gin.Default()

// 	r.Run(":8080") //!TODO Make the port config client adjustable.
// }

// func mongoInit(uri string) *mongo.Database {
// 	client, err := mongo.Connect(context.TODO(), options.Client().ApplyURI(uri))

// 	if err != nil {
// 		panic(err) //!TODO throw more sustainable errors
// 	}

// 	na := client.Database("nancy-account")
// 	return na
// }
