package config

import "github.com/cbroglie/mustache"

type Template struct {
	*mustache.Template
}

func parseStringTemplate(template string) (*Template, error) {

	temp, err := mustache.ParseString(template)

	return &Template{
		temp,
	}, err

}

func parseFileTemplate(file string) (*Template, error) {

	temp, err := mustache.ParseFile(file)

	return &Template{
		temp,
	}, err

}

func (t *Template) UnmarshalText(text []byte) error {

	temp, err := mustache.ParseString(string(text))

	t.Template = temp

	return err

}

func (t *Template) MarshalText() ([]byte, error) {

	return []byte{}, nil

}
