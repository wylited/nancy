package main

import (
	"context"

	"go.mongodb.org/mongo-driver/mongo"
	"go.mongodb.org/mongo-driver/mongo/options"
)

func initMongo(uri string) *mongo.Client {
	client, err := mongo.Connect(context.TODO(), options.Client().ApplyURI(uri))

	if err != nil {
		panic(err) //!TODO throw more sustainable errors
	}

	return client
}

type Account struct {
	ID      string `json:"id"`
	Type    string `json:"type"`
	Name    string `json:"name"`
	Balance string `json:"balance"`
}

type Transaction struct {
	ID       string `json:"id"`
	Account  string `json:"account"`
	Date     string `json:"date"`
	Payee    string `json:"payee"`
	Credit   string `json:"credit"`
	Debit    string `json:"debit"`
	Notes    string `json:"notes"`
	Category string `json:"category"`
}

type Category struct {
	ID      string   `json:"id"`
	Name    string   `json:"name"`
	Parent  string   `json:"parent"`
	Subcats []string `json:"subcats"`
}

// type Type string
// const (
// 	checking Type = "checking"
// 	savings  Type = "savings"
// 	card     Type = "card"
// 	invest   Type = "investment"
// 	mortage  Type = "mortage"
// 	other    Type = "other"
// )
