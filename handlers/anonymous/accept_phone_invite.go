package anonymous

import (
	"github.com/sirupsen/logrus"
	"github.com/zolamk/trust/config"
	"github.com/zolamk/trust/handlers"
	"github.com/zolamk/trust/middleware"
	"github.com/zolamk/trust/model"
	"gorm.io/gorm"
)

func AcceptPhoneInvite(db *gorm.DB, c *config.Config, token string, password string, log_data *middleware.LogData) (*model.User, error) {

	user := &model.User{}

	if tx := db.First(user, "phone_invitation_token = ?", token); tx.Error != nil {

		if tx.Error == gorm.ErrRecordNotFound {
			return nil, handlers.ErrUserNotFound
		}

		logrus.Error(tx.Error)

		return nil, handlers.ErrInternal

	}

	user.SetPassword(password, int(c.PasswordHashCost))

	log := model.NewLog(user.ID, "accepted phone invitation", log_data.IP, nil, log_data.Location, log_data.UserAgent)

	if err := user.AcceptPhoneInvite(db, log); err != nil {
		logrus.Error(err)
		return nil, handlers.ErrInternal
	}

	return user, nil

}
