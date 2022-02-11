package compilers

import (
	"testing"

	"github.com/stretchr/testify/assert"
	"github.com/zolamk/trust/model"
)

func TestCompileQuery(t *testing.T) {

	assert := assert.New(t)

	fields := []string{"id", "email"}

	where := map[string]interface{}{}

	order_by := []model.Object{}

	query, params, err := CompileQuery(fields, where, order_by, 0, 100)

	assert.Equal(nil, err, "expected err to be nil but got %v", err)

	assert.Equal(1, len(params), "expected 2 parameters to be returned but got %d", len(params))

	expected_query := `SELECT "id", "email" FROM "trust"."users" LIMIT ?`

	assert.Equal(expected_query, *query, "expected query %s but got %s", expected_query, *query)

	assert.Equal(int64(100), params[0], "expected parameter to be 100 but got %d", params[0])

	query, params, err = CompileQuery(fields, where, order_by, 10, 100)

	assert.Equal(nil, err, "expected err to be nil but got %v", err)

	expected_query = `SELECT "id", "email" FROM "trust"."users" LIMIT ? OFFSET ?`

	assert.Equal(expected_query, *query, "expected query %s but got %s", expected_query, *query)

	assert.Equal(int64(100), params[0], "expected parameter to be 100 but got %d", params[0])

	assert.Equal(int64(10), params[1], "expected parameter to be 10 but got %d", params[0])

	order_by = []model.Object{
		{
			"id": "asc",
		},
		{
			"email": "desc",
		},
	}

	where = map[string]interface{}{
		"id": map[string]interface{}{
			"_eq": "ID",
		},
	}

	query, params, err = CompileQuery(fields, where, order_by, 10, 100)

	assert.Equal(nil, err, "expected err to be nil but got %v", err)

	expected_query = `SELECT "id", "email" FROM "trust"."users" WHERE ("id" = ?) ORDER BY "id" ASC, "email" DESC LIMIT ? OFFSET ?`

	assert.Equal(expected_query, *query, "expected query %s but got %s", expected_query, *query)

	assert.Equal("ID", params[0], "expected parameter to be 100 but got %d", params[0])

	assert.Equal(int64(100), params[1], "expected parameter to be 100 but got %d", params[0])

	assert.Equal(int64(10), params[2], "expected parameter to be 10 but got %d", params[0])

}
