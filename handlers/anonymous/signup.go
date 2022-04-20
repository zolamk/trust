package anonymous

import (
	"github.com/sirupsen/logrus"
	"github.com/zolamk/trust/config"
	"github.com/zolamk/trust/handlers"
	"github.com/zolamk/trust/model"
	"gorm.io/gorm"
)

func Signup(db *gorm.DB, config *config.Config, form model.SignupForm) (*model.User, error) {

	err := form.Data.Validate(config.CustomDataSchema)

	if err != nil {

		logrus.Error(err)

		return nil, handlers.ErrObjectDoesntMatchSchema

	}

	if !config.PasswordRule.MatchString(form.Password) {

		return nil, handlers.ErrInvalidPassword

	}

	user := &model.User{
		Name:   form.Name,
		Email:  form.Email,
		Phone:  form.Phone,
		Avatar: form.Avatar,
		Data:   &form.Data,
	}

	if form.Email != nil {

		if err := handlers.ValidateEmail(*form.Email, db, config); err != nil {

			return nil, err

		}

	}

	if form.Phone != nil {

		if err := handlers.ValidatePhone(*form.Phone, db, config); err != nil {

			return nil, err
		}

	}

	err = db.Transaction(func(tx *gorm.DB) error {

		if err := user.SetPassword(form.Password, int(config.PasswordHashCost)); err != nil {

			logrus.Error(err)

			return handlers.ErrInternal

		}

		if err = user.Create(tx); err != nil {

			logrus.Error(err)

			return handlers.ErrInternal

		}

		if user.Email != nil {

			if err := handlers.SendEmailConfirmation(user, tx, config); err != nil {

				logrus.Error(err)

				return handlers.ErrInternal

			}

		}

		if user.Phone != nil {

			if err := handlers.SendPhoneConfirmation(user, tx, config); err != nil {

				logrus.Error(err)

				return handlers.ErrInternal

			}

		}

		return nil

	})

	if err != nil {
		return nil, err
	}

	return user, nil

}
