package users

import (
	"github.com/sirupsen/logrus"
	"github.com/zolamk/trust/config"
	"github.com/zolamk/trust/handlers"
	"github.com/zolamk/trust/jwt"
	"github.com/zolamk/trust/middleware"
	"github.com/zolamk/trust/model"
	"gorm.io/gorm"
)

func confirm(user *model.User, ip, ua, adminID string, tx *gorm.DB) error {

	if user.Email != nil {

		log := model.NewLog(user.ID, "email confirmed by admin", ip, &adminID, ua)

		if err := user.ConfirmEmail(tx, log); err != nil {

			logrus.Error(err)

			return handlers.ErrInternal

		}

	}

	if user.Phone != nil {

		log := model.NewLog(user.ID, "phone confirmed by admin", ip, &adminID, ua)

		if err := user.ConfirmPhone(tx, log); err != nil {

			logrus.Error(err)

			return handlers.ErrInternal

		}

	}

	return nil

}

func validateForm(form model.CreateUserForm, config *config.Config) error {

	if form.Email == nil && form.Phone == nil {

		return handlers.ErrEmailOrPhoneRequired

	}

	if form.Email != nil && config.DisableEmail {

		return handlers.ErrEmailDisabled

	}

	if form.Phone != nil && config.DisablePhone {

		return handlers.ErrPhoneDisabled

	}

	if err := form.Data.Validate(config.CustomDataSchema); err != nil {

		e := handlers.ErrObjectDoesntMatchSchema

		e.Extensions["message"] = err.Error()

		return e

	}

	return nil

}

func CreateUser(db *gorm.DB, config *config.Config, token *jwt.JWT, form model.CreateUserForm, log_data *middleware.LogData) (*model.User, error) {

	var err error

	is_admin := token.HasAdminRole()

	if !is_admin {

		return nil, handlers.ErrAdminOnly

	}

	if err = validateForm(form, config); err != nil {

		return nil, err

	}

	user := &model.User{
		Name:   form.Name,
		Avatar: form.Avatar,
		Email:  form.Email,
		Phone:  form.Phone,
		Data:   &form.Data,
	}

	if form.Password != nil {

		if !config.PasswordRule.MatchString(*form.Password) {

			return nil, handlers.ErrInvalidPassword

		}

		if err := user.SetPassword(*form.Password, int(config.PasswordHashCost)); err != nil {

			logrus.Error(err)

			return nil, handlers.ErrInternal

		}

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

		log := model.NewLog(user.ID, "created by admin", log_data.IP, &token.Subject, log_data.UserAgent)

		if err := user.CreateWithLog(tx, &log); err != nil {

			logrus.Error(err)

			return handlers.ErrInternal

		}

		if form.Confirm != nil && *form.Confirm {

			if err := confirm(user, log_data.IP, log_data.UserAgent, token.Subject, tx); err != nil {

				return err

			}

			return nil

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
