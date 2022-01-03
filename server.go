package main

import (
	"embed"
	"os"

	_ "github.com/golang-migrate/migrate/v4/database/postgres"
	"github.com/sirupsen/logrus"
	"github.com/urfave/cli"
	"github.com/zolamk/trust/cmd"
)

//go:embed migrations/*.sql
var files embed.FS

func main() {

	app := &cli.App{
		Name:        "trust",
		Description: "Trust is a user registration and authentication GraphQL API",
		Commands: []cli.Command{
			{
				Name:  "run",
				Usage: "Start trust server",
				Flags: []cli.Flag{
					cli.StringFlag{
						Name:     "config",
						Usage:    "Configuration file path",
						Required: true,
					},
				},
				Action: func(c *cli.Context) {
					cmd.Run(c.String("config"), files)
				},
			},
		},
	}

	logrus.Fatalln(app.Run(os.Args))

}
