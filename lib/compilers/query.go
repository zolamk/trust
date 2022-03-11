package compilers

import (
	"github.com/doug-martin/goqu/v9"
	"github.com/doug-martin/goqu/v9/exp"
	"github.com/zolamk/trust/model"
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

func getMapping(op string, value interface{}) (string, interface{}) {

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

	return op, value

}

func andOps(key string, value interface{}) exp.ExpressionList {

	ands := goqu.And()

	for op, v := range value.(map[string]interface{}) {

		op, v = getMapping(op, v)

		ands = ands.Append(goqu.Ex{
			key: goqu.Op{op: v},
		})

	}

	return ands

}

func compileOr(where []interface{}) exp.ExpressionList {

	or := goqu.Or()

	for _, value := range where {

		ands := goqu.And()

		for key, value := range value.(map[string]interface{}) {

			if key == "_or" {

				var sub_ors exp.ExpressionList

				switch v := value.(type) {
				case []interface{}:
					sub_ors = compileOr(v)
				// handle graphql list coercion
				case []map[string]interface{}:
					sub_ors = compileOr([]interface{}{v[0]})
				}

				ands = ands.Append(sub_ors)

				continue

			}

			ands = ands.Append(andOps(key, value))

		}

		or = or.Append(ands)

	}

	return or

}

func CompileWhere(where map[string]interface{}) exp.ExpressionList {

	operations := goqu.And()

	for key, value := range where {

		if key == "_or" {

			var or exp.ExpressionList

			switch v := value.(type) {
			case []interface{}:
				or = compileOr(v)
			// handle graphql list coercion
			case []map[string]interface{}:
				or = compileOr([]interface{}{v[0]})
			}

			operations = operations.Append(or)

			continue

		}

		ands := andOps(key, value)

		operations = operations.Append(ands)

	}

	return operations

}

func CompileQuery(fields []string, where map[string]interface{}, order_by []model.Object, offset, limit int) (*string, []interface{}, error) {

	i_fields := make([]interface{}, len(fields))

	for _, field := range fields {
		i_fields = append(i_fields, field)
	}

	builder := goqu.From("trust.users").Prepared(true).Select(i_fields...)

	operations := CompileWhere(where)

	builder = builder.Where(operations)

	for _, value := range order_by {

		for key, value := range value {

			if value == "asc" {
				builder = builder.OrderAppend(goqu.I(key).Asc())
				continue
			}

			builder = builder.OrderAppend(goqu.I(key).Desc())

		}

	}

	builder = builder.Offset(uint(offset)).Limit(uint(limit))

	sql, params, err := builder.ToSQL()

	return &sql, params, err

}
