package model

import (
	"time"

	"gorm.io/gorm"
)

type Log struct {
	UserID    string
	Event     string
	At        time.Time
	IPAddress string
	UserAgent string
	AdminID   *string
}

func (l *Log) Create(tx *gorm.DB) error {

	return tx.Create(l).Error

}

func NewLog(user_id string, event string, ip string, admin_id *string, ua string) Log {

	return Log{
		user_id,
		event,
		time.Now(),
		ip,
		ua,
		admin_id,
	}
}
