package main

import (
	"context"

	"go.mongodb.org/mongo-driver/mongo"
	"go.mongodb.org/mongo-driver/mongo/options"
	"go.mongodb.org/mongo-driver/mongo/readpref"
)

func main() {
	// Connect to the mongodb server
	client, err := mongo.Connect(context.TODO(), options.Client().ApplyURI("mongodb://localhost:27017"))
	if err != nil {
		panic(err)
	}

	// Check the connection
	if err := client.Ping(context.TODO(), readpref.Primary()); err != nil {
		panic(err)
	}

	// usersCollection := client.Database("testing").Collection("users")
}
