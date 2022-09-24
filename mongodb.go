package main

import (
	"context"
	"log"

	"go.mongodb.org/mongo-driver/bson"
	"go.mongodb.org/mongo-driver/mongo"
	"go.mongodb.org/mongo-driver/mongo/options"
)

type Account struct {
	Name    string `json:"name"`
	Type    atype  `json:"type"`
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
	SubCats []string `json:"subcats"`
}

type atype string

const (
	checking atype = "checking"
	savings  atype = "savings"
	card     atype = "card"
	invest   atype = "investment"
	mortage  atype = "mortage"
	other    atype = "other"
)

func initMongo(uri string) *mongo.Client {
	client, err := mongo.Connect(context.TODO(), options.Client().ApplyURI(uri))

	if err != nil {
		panic(err) //!TODO throw more sustainable errors
	}

	return client
}

// listAccounts returns a list of all accounts in the database
func listAccounts(client *mongo.Client) []Account {
	accounts := client.Database("configuration").Collection("accounts")

	cur, err := accounts.Find(context.TODO(), bson.D{})
	if err != nil {
		log.Fatal(err)
	}

	var accountList []Account
	if err = cur.All(context.TODO(), &accountList); err != nil {
		log.Fatal(err)
	}

	return accountList
}

// getAccount returns a single account from the database
func getAccount(client *mongo.Client, name string) Account {
	accounts := client.Database("configuration").Collection("accounts")

	var account Account
	if err := accounts.FindOne(context.TODO(), bson.M{"name": name}).Decode(&account); err != nil {
		log.Fatal(err)
	}

	return account
}

// addAccount adds an account to the database
func addAccount(client *mongo.Client, account Account) {
	accounts := client.Database("configuration").Collection("accounts")

	if _, err := accounts.InsertOne(context.TODO(), account); err != nil {
		log.Fatal(err)
	}

	if err := client.Database("accounts").CreateCollection(context.TODO(), account.Name); err != nil {
		log.Fatal(err)
	}
}

// deleteAccount deletes an account from the database
func deleteAccount(client *mongo.Client, name string) {
	accounts := client.Database("configuration").Collection("accounts")

	if _, err := accounts.DeleteOne(context.TODO(), bson.M{"name": name}); err != nil {
		log.Fatal(err)
	}

	// leave double checking if they want to drop up to the client side.
	if err := client.Database("accounts").Collection(name).Drop(context.TODO()); err != nil {
		log.Fatal(err)
	}
}

// updateAccount updates an account in the database
// Can't update the name of the account
func updateAccount(client *mongo.Client, name string, account Account) {
	accounts := client.Database("configuration").Collection("accounts")

	if _, err := accounts.UpdateOne(context.TODO(), bson.M{"name": name}, bson.M{"$set": account}); err != nil {
		log.Fatal(err)
	}
}

// getAccountatype returns the atype of an account
func getAccountatype(client *mongo.Client, name string) atype {
	accounts := client.Database("configuration").Collection("accounts")

	var account Account
	if err := accounts.FindOne(context.TODO(), bson.M{"name": name}).Decode(&account); err != nil {
		log.Fatal(err)
	}

	return account.Type
}

// getAccountBalance returns the balance of an account
func getAccountBalance(client *mongo.Client, name string) string {
	accounts := client.Database("configuration").Collection("accounts")

	var account Account
	if err := accounts.FindOne(context.TODO(), bson.M{"name": name}).Decode(&account); err != nil {
		log.Fatal(err)
	}

	return account.Balance
}
