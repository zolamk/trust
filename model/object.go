package model

import (
	"database/sql/driver"
	"encoding/json"
	"fmt"
	"io"

	"github.com/zolamk/trust/config"
	"gorm.io/gorm"
	"gorm.io/gorm/schema"
)

type Object map[string]interface{}

func (o Object) MarshalGQL(w io.Writer) {

	encoder := json.NewEncoder(w)

	if err := encoder.Encode(o); err != nil {
		panic(err)
	}

}

func (o *Object) UnmarshalGQL(v interface{}) error {

	m, ok := v.(map[string]interface{})

	if !ok {

		return fmt.Errorf("%T is not a map", v)

	}

	if m == nil {

		*o = nil

		return nil

	}

	*o = Object(m)

	return nil

}

func (o *Object) Scan(v interface{}) error {

	bytes, ok := v.([]byte)

	if !ok {
		return fmt.Errorf("failed to unmarshal jsonb value: %T", v)
	}

	obj := Object{}

	err := json.Unmarshal(bytes, &obj)

	*o = obj

	return err

}

func (o Object) Value() (driver.Value, error) {

	if len(o) == 0 {
		return nil, nil
	}

	return json.Marshal(o)

}

func (Object) GormDataType() string {
	return "json"
}

func (Object) GormDBDataType(db *gorm.DB, field *schema.Field) string {
	switch db.Dialector.Name() {
	case "sqlite":
		return "JSON"
	case "mysql":
		return "JSON"
	case "postgres":
		return "JSONB"
	}
	return ""
}

func (o Object) Validate(schema map[string]config.Field) error {

	for key := range o {

		if _, ok := schema[key]; !ok {

			return fmt.Errorf("unexpected field %v included in data", key)

		}

	}

	for name, field := range schema {

		if err := field.Validate(name, o[name]); err != nil {
			return err
		}

	}

	return nil

}
