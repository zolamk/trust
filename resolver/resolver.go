package resolver

import (
	"github.com/zolamk/trust/config"
	"github.com/zolamk/trust/graphjin/sdata"
	"gorm.io/gorm"
)

// This file will not be regenerated automatically.
//
// It serves as dependency injection for your app, add any dependencies you require here.

type Resolver struct {
	DB       *gorm.DB
	Config   *config.Config
	DBSchema *sdata.DBSchema
}
