package model

import (
	"time"

	"gorm.io/gorm"
)

type RefreshToken struct {
	ID        uint64 `gorm:"autoIncrement"`
	Token     string
	UserID    string
	CreatedAt time.Time
	UpdatedAt *time.Time
}

func (r *RefreshToken) Create(tx *gorm.DB) error {
	return tx.Create(r).Error
}
