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
	_ "github.com/golang-migrate/migrate/v4/source/iofs"
	"github.com/gorilla/mux"
	"github.com/ip2location/ip2location-go/v9"
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

	config := config.New()

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

	ip2location_db, err := ip2location.OpenDB(config.IP2LocationDBPath)

	if err != nil {
		logrus.Fatalln(err)
	}

	defer ip2location_db.Close()

	graphql := handler.NewDefaultServer(generated.NewExecutableSchema(generated.Config{Resolvers: &resolver.Resolver{
		DB:            db,
		Config:        config,
		IP2LocationDB: ip2location_db,
	}}))

	router := mux.NewRouter()

	router.Handle("/graphiql", playground.Handler("GraphQL playground", "/graphql")).Methods("GET")

	router.Handle("/graphql",
		middleware.Headers(ip2location_db)(
			middleware.Response(
				middleware.ExtractRefresh(config)(
					middleware.Authenticated(config)(graphql),
				),
			),
		),
	).Methods("POST")

	router.Handle("/authorize", provider.Authorize(db, config)).Methods("GET")

	router.Handle("/authorize/callback",
		middleware.Headers(ip2location_db)(
			provider.Callback(db, config, ip2location_db),
		),
	).Methods("GET")

	http.Handle("/", router)

	host := fmt.Sprintf("%s:%d", config.Host, config.Port)

	logrus.WithField("host", host).Info("started trust server")

	log.Fatal(http.ListenAndServe(host, nil))

}
