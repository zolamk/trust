package compilers

import (
	"fmt"
	"strings"
)

func compileSubWhere(key string, value map[string]interface{}, params_count int) (string, []interface{}) {

	var operations []string

	var params []interface{}

	for op, arg := range value {
		switch op {
		case "_eq":
			params_count = params_count + 1
			operation := fmt.Sprintf(`"%s" = $%d`, key, params_count)
			params = append(params, arg)
			operations = append(operations, operation)
		case "_neq":
			params_count = params_count + 1
			operation := fmt.Sprintf(`"%s" != $%d`, key, params_count)
			params = append(params, arg)
			operations = append(operations, operation)
		case "_lt":
			params_count = params_count + 1
			operation := fmt.Sprintf(`"%s" < $%d`, key, params_count)
			params = append(params, arg)
			operations = append(operations, operation)
		case "_lte":
			params_count = params_count + 1
			operation := fmt.Sprintf(`"%s" <= $%d`, key, params_count)
			params = append(params, arg)
			operations = append(operations, operation)
		case "_gt":
			params_count = params_count + 1
			operation := fmt.Sprintf(`"%s" >= $%d`, key, params_count)
			params = append(params, arg)
			operations = append(operations, operation)
		case "_gte":
			params_count = params_count + 1
			operation := fmt.Sprintf(`"%s" <= $%d`, key, params_count)
			params = append(params, arg)
			operations = append(operations, operation)
		case "_like":
			params_count = params_count + 1
			operation := fmt.Sprintf(`"%s" LIKE $%d`, key, params_count)
			params = append(params, arg)
			operations = append(operations, operation)
		case "_ilike":
			params_count = params_count + 1
			operation := fmt.Sprintf(`"%s" ILIKE $%d`, key, params_count)
			params = append(params, arg)
			operations = append(operations, operation)
		case "_nlike":
			params_count = params_count + 1
			operation := fmt.Sprintf(`"%s" NLIKE $%d`, key, params_count)
			params = append(params, arg)
			operations = append(operations, operation)
		case "_nilike":
			params_count = params_count + 1
			operation := fmt.Sprintf(`"%s" NILIKE $%d`, key, params_count)
			params = append(params, arg)
			operations = append(operations, operation)
		case "_is_null":

			operation := fmt.Sprintf(`"%s" IS NOT NULL`, key)

			if arg.(bool) {
				operation = fmt.Sprintf(`"%s" IS NULL`, key)
			}

			operations = append(operations, operation)

		}

	}

	if len(operations) == 1 {
		return operations[0], params
	}

	if len(operations) > 1 {

		return strings.Join(operations, " AND "), params

	}

	return "", params

}

func compileWhere(where map[string]interface{}) (string, []interface{}, error) {

	var operations []string

	var params []interface{}

	for key, value := range where {

		sub_where, sub_params := compileSubWhere(key, value.(map[string]interface{}), len(params))

		operations = append(operations, sub_where)

		params = append(params, sub_params...)

	}

	var clause string

	if len(operations) == 1 {
		clause = operations[0]
	}

	if len(operations) > 1 {

		clause = strings.Join(operations, " AND ")

	}

	return clause, params, nil

}
