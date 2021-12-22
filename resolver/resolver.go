package resolver

import (
	"github.com/ip2location/ip2location-go/v9"
	"github.com/zolamk/trust/config"
	"gorm.io/gorm"
)

// This file will not be regenerated automatically.
//
// It serves as dependency injection for your app, add any dependencies you require here.

type Resolver struct {
	DB            *gorm.DB
	Config        *config.Config
	IP2LocationDB *ip2location.DB
}
