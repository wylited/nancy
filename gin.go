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
	r.GET("/api/accounts", func(c *gin.Context) {
		accounts, err := listAccounts(m)
		if err != nil {
			c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
			return
		}
		c.JSON(http.StatusOK, accounts)
	})

	// Get an account from MongoDB
	r.GET("/api/accounts/:id", func(c *gin.Context) {
		id := c.Param("id")
		account, err := getAccount(m, id, "all")
		if err != nil {
			c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
			return
		}
		c.JSON(http.StatusOK, account)
	})

	// Get an account's type from MongoDB
	r.GET("/api/accounts/:id/type", func(c *gin.Context) {
		id := c.Param("id")
		account, err := getAccount(m, id, "type")
		if err != nil {
			c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
			return
		}
		c.JSON(http.StatusOK, account)
	})

	// Get an account's balance from MongoDB
	r.GET("/api/accounts/:id/balance", func(c *gin.Context) {
		id := c.Param("id")
		account, err := getAccount(m, id, "balance")
		if err != nil {
			c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
			return
		}
		c.JSON(http.StatusOK, account)
	})

	// Add an account to MongoDB
	r.POST("/api/accounts", func(c *gin.Context) {
		var account Account
		if err := c.BindJSON(&account); err != nil {
			c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
			return
		}
		if err := addAccount(m, account); err != nil {
			c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
			return
		}
		c.JSON(http.StatusOK, account)
	})

	// Delete an account from MongoDB
	r.DELETE("/api/accounts/:id", func(c *gin.Context) {
		id := c.Param("id")
		if err := deleteAccount(m, id); err != nil {
			c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
			return
		}
		c.JSON(http.StatusOK, gin.H{"id": id})
	})

	// Update an account in MongoDB
	r.PUT("/api/accounts/:id", func(c *gin.Context) {
		id := c.Param("id")
		var account Account
		if err := c.BindJSON(&account); err != nil {
			c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
			return
		}
		if err := updateAccount(m, id, account); err != nil {
			c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
			return
		}
		c.JSON(http.StatusOK, account)
	})

	// Update an account's type in MongoDB
	r.PUT("/api/accounts/:id/type", func(c *gin.Context) {
		id := c.Param("id")
		var account Account
		if err := c.BindJSON(&account); err != nil {
			c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
			return
		}
		if err := updateAccount(m, id, account); err != nil {
			c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
			return
		}
		c.JSON(http.StatusOK, account)
	})

	// Update an account's balance in MongoDB
	r.PUT("/api/accounts/:id/balance", func(c *gin.Context) {
		id := c.Param("id")
		var account Account
		if err := c.BindJSON(&account); err != nil {
			c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
			return
		}
		if err := updateAccount(m, id, account); err != nil {
			c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
			return
		}
		c.JSON(http.StatusOK, account)
	})
}
