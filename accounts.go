package main

import (
	"context"
	"log"

	"go.mongodb.org/mongo-driver/bson"
	"go.mongodb.org/mongo-driver/bson/primitive"
	"go.mongodb.org/mongo-driver/mongo"
)

type Account struct {
	id      string               `json:"id"`
	Name    string               `json:"name"`
	Type    string               `json:"type"`
	Balance primitive.Decimal128 `json:"balance"`
}

// Return a list of all accounts
func listAccounts(client *mongo.Client) (acc []Account, err error) {
	accounts := client.Database("configuration").Collection("accounts")
	var accountList []Account

	cursor, err := accounts.Find(context.TODO(), bson.D{})
	if err != nil {
		log.Println(err)
		return nil, err
	}
	if err = cursor.All(context.TODO(), &accountList); err != nil {
		log.Println(err)
		return nil, err
	}

	return accountList, nil
}

// Return an account
func getAccount(client *mongo.Client, name string, part string) (acc Account, err error) {
	accounts := client.Database("configuration").Collection("accounts")

	var account Account
	if err := accounts.FindOne(context.TODO(), bson.M{"name": name}).Decode(&account); err != nil {
		log.Println(err)
		return account, err
	}

	if part == "id" {
		return Account{id: account.id}, nil
	} else if part == "type" {
		return Account{Type: account.Type}, nil
	} else if part == "balance" {
		return Account{Balance: account.Balance}, nil
	} else {
		return account, nil
	}

}

// Adds an account to MongoDB and returns an
// addAccount adds an account to the database
func addAccount(client *mongo.Client, account Account) (err error) {
	accounts := client.Database("configuration").Collection("accounts")

	if _, err := accounts.InsertOne(context.TODO(), account); err != nil {
		return err
	}

	if err := client.Database("accounts").CreateCollection(context.TODO(), account.Name); err != nil {
		return err
	}

	return nil
}

// updateAccount updates an account in the database
// Can't update the id of the account, yet
func updateAccount(client *mongo.Client, id string, account Account) (err error) {
	accounts := client.Database("configuration").Collection("accounts")

	if _, err := accounts.UpdateOne(context.TODO(), bson.M{"id": id}, bson.M{"$set": account}); err != nil {
		return err
	}

	return nil
}

// deleteAccount deletes an account from the database
func deleteAccount(client *mongo.Client, id string) (err error) {
	accounts := client.Database("configuration").Collection("accounts")

	if _, err := accounts.DeleteOne(context.TODO(), bson.M{"_id": id}); err != nil {
		return err
	}

	// leave double checking if they want to drop up to the client side.
	if err := client.Database("accounts").Collection(id).Drop(context.TODO()); err != nil {
		return err
	}

	return nil
}
