package mail

import (
	"github.com/zolamk/trust/config"
	"gopkg.in/gomail.v2"
)

func SendEmail(template *config.TemplateConfig, context map[string]string, to string, config *config.SMTPConfig) error {

	mailer := gomail.NewDialer(config.Host, int(config.Port), config.Username, config.Password)

	mail := gomail.NewMessage()

	mail.SetHeader("From", config.Email)

	mail.SetHeader("To", to)

	mail.SetHeader("Subject", template.Subject)

	body, err := template.Email.Render(context)

	if err != nil {
		return err
	}

	mail.SetBody("text/html", body)

	return mailer.DialAndSend(mail)

}
