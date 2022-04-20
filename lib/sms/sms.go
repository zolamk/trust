package sms

import (
	"bytes"
	"encoding/json"
	"net/http"

	"github.com/zolamk/trust/config"
)

func SendSMS(template *config.TemplateConfig, to string, context map[string]string, config *config.SMSConfig) error {

	var message string

	var err error

	var req *http.Request

	var data []byte

	if message, err = template.SMS.Render(context); err != nil {
		return err
	}

	body := map[string]interface{}{
		config.Mapping.Source:      config.Source,
		config.Mapping.Message:     message,
		config.Mapping.Destination: to,
	}

	for key, value := range config.Extra {
		body[key] = value
	}

	if data, err = json.Marshal(body); err != nil {
		return err
	}

	if req, err = http.NewRequest(config.Method, config.URL, bytes.NewReader(data)); err != nil {
		return err
	}

	for key, value := range config.Headers {
		req.Header.Add(key, value)
	}

	req.Header.Add("content-type", "application/json")

	_, err = http.DefaultClient.Do(req)

	return err
}
