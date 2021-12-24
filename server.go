package main

import (
	"embed"
	"fmt"
	"log"
	"net/http"
	"os"

	"github.com/99designs/gqlgen/graphql/handler"
	"github.com/99designs/gqlgen/graphql/playground"
	"github.com/golang-migrate/migrate/v4"
	_ "github.com/golang-migrate/migrate/v4/database/postgres"
	"github.com/golang-migrate/migrate/v4/source/iofs"
	"github.com/gorilla/mux"
	"github.com/sirupsen/logrus"
	"github.com/zolamk/trust/config"
	"github.com/zolamk/trust/graph/generated"
	"github.com/zolamk/trust/handlers/provider"
	"github.com/zolamk/trust/middleware"
	"github.com/zolamk/trust/resolver"
	"gorm.io/driver/postgres"
	"gorm.io/gorm"
	"gorm.io/gorm/logger"
	"gorm.io/gorm/schema"
)

//go:embed migrations/*.sql
var files embed.FS

func main() {

	config, err := config.New("./.conf")

	if err != nil {
		log.Fatalln(err)
	}

	logrus.SetFormatter(&logrus.JSONFormatter{})

	logrus.SetOutput(os.Stdout)

	logrus.SetLevel(config.LogLevel)

	migrations, err := iofs.New(files, "migrations")

	if err != nil {
		logrus.Fatalln(err)
	}

	db, err := gorm.Open(postgres.Open(config.DatabaseURL), &gorm.Config{
		NamingStrategy: schema.NamingStrategy{
			TablePrefix:   "trust.",
			SingularTable: false,
		},
		Logger: logger.Default.LogMode(logger.Silent),
	})

	if err != nil {
		logrus.Fatalln(err)
	}

	sql, err := db.DB()

	if err != nil {
		logrus.Fatalln(err)
	}

	sql.SetMaxIdleConns(config.MaxConnectionPoolSize)

	sql.SetMaxOpenConns(config.MaxConnectionPoolSize)

	migration_driver, err := migrate.NewWithSourceInstance("iofs", migrations, config.DatabaseURL)

	if err != nil {
		logrus.Fatalln(err)
	}

	if err := migration_driver.Up(); err != nil && err != migrate.ErrNoChange {
		logrus.Fatalln(err)
	}

	graphql := handler.NewDefaultServer(generated.NewExecutableSchema(generated.Config{Resolvers: &resolver.Resolver{
		DB:     db,
		Config: config,
	}}))

	router := mux.NewRouter()

	router.Use(middleware.AttachResponse)

	router.Use(middleware.AttachLogData(config.IP2LocationDB))

	router.Use(middleware.AttachRefreshToken(config))

	router.Use(middleware.Authenticated(config))

	router.Handle("/graphiql", playground.Handler("GraphQL playground", "/graphql")).Methods("GET")

	router.Handle("/graphql", graphql).Methods("POST")

	router.Handle("/authorize", provider.Authorize(db, config)).Methods("GET")

	router.Handle("/authorize/callback", provider.Callback(db, config)).Methods("GET")

	http.Handle("/", router)

	host := fmt.Sprintf("%s:%d", config.Host, config.Port)

	logrus.WithField("host", host).Info("started trust server")

	log.Fatal(http.ListenAndServe(host, nil))

}
