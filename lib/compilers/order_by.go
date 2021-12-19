package compilers

import (
	"fmt"
	"strings"
)

func compileOrderBy(order_by map[string]interface{}) string {

	var operations []string

	for key, value := range order_by {

		operation := fmt.Sprintf(`"%s" %s`, key, value)

		operations = append(operations, operation)

	}

	return strings.Join(operations, ", ")

}
