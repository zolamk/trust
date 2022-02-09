package compilers

import (
	"github.com/doug-martin/goqu/v9"
)

var mapping = map[string]string{
	"_eq":      "eq",
	"_neq":     "neq",
	"_gt":      "gt",
	"_gte":     "get",
	"_like":    "like",
	"_ilike":   "ilike",
	"_nlike":   "notlike",
	"_nilike":  "notilike",
	"_lt":      "lt",
	"_lte":     "lte",
	"_is_null": "_is_null",
}

func CompileQuery(fields []string, where map[string]interface{}, order_by map[string]interface{}, offset, limit int) (*string, []interface{}, error) {

	i_fields := make([]interface{}, len(fields))

	for _, field := range fields {
		i_fields = append(i_fields, field)
	}

	builder := goqu.From("trust.users").Prepared(true).Select(i_fields...)

	operations := []goqu.Expression{}

	for key, value := range where {

		for op, value := range value.(map[string]interface{}) {

			op = mapping[op]

			// since goqu doesn't handle is_null expressions
			// convert to _eq and _neq until support is provided
			// by goqu
			if op == "_is_null" {

				if value.(bool) {

					op = "eq"

					value = nil

				} else {

					op = "neq"

					value = nil

				}

			}

			operations = append(operations, goqu.Ex{
				key: goqu.Op{op: value},
			})

		}

	}

	builder = builder.Where(operations...)

	for key, value := range order_by {

		if value == "asc" {
			builder = builder.OrderAppend(goqu.I(key).Asc())
			continue
		}

		builder = builder.OrderAppend(goqu.I(key).Desc())

	}

	builder = builder.Offset(uint(offset)).Limit(uint(limit))

	sql, params, err := builder.ToSQL()

	return &sql, params, err

}
