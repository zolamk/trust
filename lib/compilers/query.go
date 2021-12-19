package compilers

import (
	"fmt"
	"strings"
)

func CompileQuery(fields []string, where map[string]interface{}, order_by map[string]interface{}, offset, limit int) (*string, []interface{}, error) {

	var query strings.Builder

	query.WriteString(`SELECT `)

	fields_count := len(fields) - 1

	for i, field := range fields {

		if i == fields_count {
			query.WriteString(fmt.Sprintf(`"%s"`, field))
			continue
		}

		query.WriteString(fmt.Sprintf(`"%s", `, field))
	}

	query.WriteString(` FROM "trust"."users"`)

	if len(where) > 0 {
		query.WriteString(` WHERE `)
	}

	where_clause, params, err := compileWhere(where)

	if err != nil {
		return nil, []interface{}{}, err
	}

	query.WriteString(where_clause)

	if len(order_by) > 0 {

		order_by_clause := compileOrderBy(order_by)

		query.WriteString(` ORDER BY `)

		query.WriteString(order_by_clause)

	}

	params_count := len(params)

	query.WriteString(fmt.Sprintf(` OFFSET $%d LIMIT $%d`, params_count+1, params_count+2))

	params = append(params, offset, limit)

	sql := query.String()

	return &sql, params, nil

}
