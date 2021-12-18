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
	"github.com/sirupsen/logrus"
	"github.com/zolamk/trust/config"
	"github.com/zolamk/trust/graph/generated"
	"github.com/zolamk/trust/graphjin/sdata"
	"github.com/zolamk/trust/middleware"
	"github.com/zolamk/trust/resolver"
	"gorm.io/driver/postgres"
	"gorm.io/gorm"
	"gorm.io/gorm/logger"
	"gorm.io/gorm/schema"
)

//go:embed migrations/*.sql
var migrations embed.FS

func main() {

	config := config.New()

	migrations, err := iofs.New(migrations, "migrations")

	logrus.SetFormatter(&logrus.JSONFormatter{})

	logrus.SetOutput(os.Stdout)

	logrus.SetLevel(config.LogLevel)

	if err != nil {
		log.Fatalln(err)
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

	db_info, err := sdata.GetDBInfo(sql, "postgres", []string{})

	if err != nil {
		logrus.Fatalln(err)
	}

	db_schema, err := sdata.NewDBSchema(db_info, map[string][]string{})

	if err != nil {
		logrus.Fatalln(err)
	}

	graphql := handler.NewDefaultServer(generated.NewExecutableSchema(generated.Config{Resolvers: &resolver.Resolver{DB: db, Config: config, DBSchema: db_schema}}))

	http.Handle("/graphiql", playground.Handler("GraphQL playground", "/graphql"))

	http.Handle("/graphql", middleware.Authenticated(config)(graphql))

	host := fmt.Sprintf("%s:%d", config.Host, config.Port)

	logrus.WithField("host", host).Info("started trust server")

	log.Fatal(http.ListenAndServe(host, nil))

}
