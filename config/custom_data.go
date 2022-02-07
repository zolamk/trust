package config

import (
	"fmt"
)

type FieldType string

const (
	String  FieldType = "string"
	Integer FieldType = "integer"
	Boolean FieldType = "boolean"
	Float   FieldType = "float"
)

type Choices []string

type Field struct {
	Type     FieldType `json:"type"`
	Required bool      `json:"required"`
	Max      *float64  `json:"max"`
	Min      *float64  `json:"min"`
	Format   *Regexp   `json:"format"`
	Choices  *Choices  `json:"choices"`
}

func (c Choices) Search(value string) bool {

	for _, v := range c {

		if v == value {
			return true
		}

	}

	return false

}

func (f Field) validateString(name string, value interface{}) error {

	switch v := value.(type) {
	case nil:
		if f.Required {
			return fmt.Errorf("field %s is required", name)
		}
	case string:
	default:
		return fmt.Errorf("field %s expected string but got %T", name, v)
	}

	v := value.(string)

	if f.Min != nil && len(v) < int(*f.Min) {
		return fmt.Errorf("field %s length is less than minimum length specified", name)
	}

	if f.Max != nil && len(v) > int(*f.Max) {
		return fmt.Errorf("field %s length is greater than maximum length specified", name)
	}

	if f.Format != nil && !f.Format.MatchString(v) {
		return fmt.Errorf("field %s doesn't match format specified", name)
	}

	if f.Choices != nil && !f.Choices.Search(v) {
		return fmt.Errorf("field %s value is not specified in choices", name)
	}

	return nil

}

func (f Field) validateInteger(name string, value interface{}) error {

	switch v := value.(type) {
	case nil:
		if f.Required {
			return fmt.Errorf("field %s is required", name)
		}
	case uint, uint8, uint16, uint32, uint64, int, int8, int16, int32, int64:
	default:
		return fmt.Errorf("field %s expected integer but got %T", name, v)
	}

	v := value.(int64)

	if f.Min != nil && v < int64(*f.Min) {
		return fmt.Errorf("field %s is less than minimum specified", name)
	}

	if f.Max != nil && v > int64(*f.Max) {
		return fmt.Errorf("field %s is greater than maximum specified", name)
	}

	return nil

}

func (f Field) validateFloat(name string, value interface{}) error {

	switch v := value.(type) {
	case nil:
		if f.Required {
			return fmt.Errorf("field %s is required", name)
		}
	case float32, float64:
	default:
		return fmt.Errorf("field %s expected float but got %T", name, v)
	}

	v := value.(float64)

	if f.Min != nil && v < *f.Min {
		return fmt.Errorf("field %s is less than minimum specified", name)
	}

	if f.Max != nil && v > *f.Max {
		return fmt.Errorf("field %s is greater than maximum specified", name)
	}

	return nil

}

func (f Field) validateBoolean(name string, value interface{}) error {

	switch v := value.(type) {
	case nil:
		if f.Required {
			return fmt.Errorf("field %s is required", name)
		}
	case bool:
	default:
		return fmt.Errorf("field %s expected boolean but got %T", name, v)
	}

	return nil

}

func (f Field) Validate(name string, value interface{}) error {

	if f.Type == String {

		return f.validateString(name, value)

	}

	if f.Type == Integer {

		return f.validateInteger(name, value)

	}

	if f.Type == Float {

		return f.validateFloat(name, value)

	}

	if f.Type == Boolean {

		return f.validateBoolean(name, value)

	}

	return fmt.Errorf("unknown field type %v", f.Type)

}
