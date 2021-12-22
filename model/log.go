package model

import (
	"time"

	"github.com/ip2location/ip2location-go/v9"
	ua "github.com/mileusna/useragent"
	"gorm.io/gorm"
)

type Log struct {
	UserID    string
	Event     string
	At        time.Time
	IPAddress string
	Country   string
	Region    string
	City      string
	ua.UserAgent
	AdminID *string
}

func (l *Log) Create(db *gorm.DB) error {
	return db.Create(l).Error
}

func NewLog(user_id string, event string, ip string, admin_id *string, location *ip2location.IP2Locationrecord, ua *ua.UserAgent) *Log {

	return &Log{
		user_id,
		event,
		time.Now(),
		ip,
		location.Country_long,
		location.Region,
		location.City,
		*ua,
		admin_id,
	}
}
