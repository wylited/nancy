package main

import (
	"context"
	"log"

	"go.mongodb.org/mongo-driver/bson"
	"go.mongodb.org/mongo-driver/mongo"
	"go.mongodb.org/mongo-driver/mongo/options"
)

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

// Return a list of previous x transactions
func listTransactions(client *mongo.Client, account string, x int64) (transactions []Transaction, err error) {
	allTransactions := client.Database(account).Collection("transactions")

	cursor, err := allTransactions.Find(context.TODO(), bson.D{}, &options.FindOptions{Limit: &x})
	if err != err {
		log.Println(err)
		return transactions, err
	}
	if err = cursor.All(context.TODO(), &transactions); err != err {
		log.Println(err)
		return transactions, err
	}
	return transactions, err
}

// Return a transaction
func getTransaction(client *mongo.Client, account string, id string, part string) (transaction Transaction, err error) {
	allTransactions := client.Database(account).Collection("transactions")

	if err := allTransactions.FindOne(context.TODO(), bson.M{"id": id}).Decode(&transaction); err != err {
		log.Println(err)
		return transaction, err
	}

	// theres gotta be a better way to write this.
	if part == "id" {
		return Transaction{ID: transaction.ID}, err
	} else if part == "account" {
		return Transaction{Account: transaction.Account}, err
	} else if part == "date" {
		return Transaction{Date: transaction.Date}, err
	} else if part == "payee" {
		return Transaction{Payee: transaction.Payee}, err
	} else if part == "credit" {
		return Transaction{Credit: transaction.Credit}, err
	} else if part == "debit" {
		return Transaction{Debit: transaction.Debit}, err
	} else if part == "notes" {
		return Transaction{Notes: transaction.Notes}, err
	} else if part == "category" {
		return Transaction{Category: transaction.Category}, err
	} else {
		return transaction, err
	}
}

// Adds a transaction to MongoDB
func addTransaction(client *mongo.Client, account string, transaction Transaction) (err error) {
	allTransactions := client.Database(account).Collection("transactions")

	if _, err := allTransactions.InsertOne(context.TODO(), transaction); err != err {
		return err
	}
	return err
}

// Updates a transaction in MongoDB
func updateTransaction(client *mongo.Client, account string, transaction Transaction, part string) (err error) {
	allTransactions := client.Database(account).Collection("transactions")

	if part == "id" {
		_, err = allTransactions.UpdateOne(context.TODO(), bson.M{"id": transaction.ID}, bson.D{{Key: "$set", Value: bson.D{{Key: "id", Value: transaction.ID}}}})
	} else if part == "account" {
		_, err = allTransactions.UpdateOne(context.TODO(), bson.M{"id": transaction.ID}, bson.D{{Key: "$set", Value: bson.D{{Key: "account", Value: transaction.Account}}}})
	} else if part == "date" {
		_, err = allTransactions.UpdateOne(context.TODO(), bson.M{"id": transaction.ID}, bson.D{{Key: "$set", Value: bson.D{{Key: "date", Value: transaction.Date}}}})
	} else if part == "payee" {
		_, err = allTransactions.UpdateOne(context.TODO(), bson.M{"id": transaction.ID}, bson.D{{Key: "$set", Value: bson.D{{Key: "payee", Value: transaction.Payee}}}})
	} else if part == "credit" {
		_, err = allTransactions.UpdateOne(context.TODO(), bson.M{"id": transaction.ID}, bson.D{{Key: "$set", Value: bson.D{{Key: "credit", Value: transaction.Credit}}}})
	} else if part == "debit" {
		_, err = allTransactions.UpdateOne(context.TODO(), bson.M{"id": transaction.ID}, bson.D{{Key: "$set", Value: bson.D{{Key: "debit", Value: transaction.Debit}}}})
	} else if part == "notes" {
		_, err = allTransactions.UpdateOne(context.TODO(), bson.M{"id": transaction.ID}, bson.D{{Key: "$set", Value: bson.D{{Key: "notes", Value: transaction.Notes}}}})
	} else if part == "category" {
		_, err = allTransactions.UpdateOne(context.TODO(), bson.M{"id": transaction.ID}, bson.D{{Key: "$set", Value: bson.D{{Key: "category", Value: transaction.Category}}}})
	} else {
		_, err = allTransactions.UpdateOne(context.TODO(),
			bson.M{"_id": transaction.ID},
			bson.D{{Key: "$set", Value: bson.D{{Key: "account", Value: transaction.Account},
				{Key: "date", Value: transaction.Date},
				{Key: "payee", Value: transaction.Payee},
				{Key: "credit", Value: transaction.Credit},
				{Key: "debit", Value: transaction.Debit},
				{Key: "notes", Value: transaction.Notes},
				{Key: "category", Value: transaction.Category}}}})}
	return err
}

// Deletes a transaction from MongoDB
func deleteTransaction(client *mongo.Client, account string, id string) (err error) {
	allTransactions := client.Database(account).Collection("transactions")

	if _, err := allTransactions.DeleteOne(context.TODO(), bson.M{"id": id}); err != err {
		return err
	}
	return err
}