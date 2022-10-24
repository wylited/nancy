package main

import (
	"context"
	
	"go.mongodb.org/mongo-driver/mongo"
	"go.mongodb.org/mongo-driver/mongo/options"
)

type Category struct {
	ID      string   `json:"id"`
	Name    string   `json:"name"`
	Parent  string   `json:"parent"`
	SubCats []string `json:"subcats"`
}

func initMongo(uri string) *mongo.Client {
	client, err := mongo.Connect(context.TODO(), options.Client().ApplyURI(uri))

	if err != nil {
		panic(err) //!TODO throw more sustainable errors
	}

	return client
}