package psql

import (
	"github.com/zolamk/trust/graphjin/qcode"
)

func (c *compilerContext) renderFunction(sel *qcode.Select, fn qcode.Function) {
	switch fn.Name {
	default:
		c.renderOtherFunction(sel, fn)
	}
	c.alias(fn.FieldName)
}

func (c *compilerContext) renderOtherFunction(sel *qcode.Select, fn qcode.Function) {
	c.w.WriteString(fn.Name)
	c.w.WriteString(`(`)
	c.colWithTable(sel.Table, fn.Col.Name)
	_, _ = c.w.WriteString(`)`)
}

func (c *compilerContext) renderBaseColumns(sel *qcode.Select) int {
	i := 0

	for _, col := range sel.BCols {
		if i != 0 {
			c.w.WriteString(`, `)
		}
		c.colWithTable(col.Col.Table, col.Col.Name)
		i++
	}
	return i
}

func (c *compilerContext) renderFunctions(sel *qcode.Select, i int) {
	for _, fn := range sel.Funcs {
		if i != 0 {
			c.w.WriteString(`, `)
		}
		c.renderFunction(sel, fn)
		i++
	}
}

func (c *compilerContext) renderRecursiveBaseColumns(sel *qcode.Select) {
	i := 0

	for _, col := range sel.Cols {
		if i != 0 {
			c.w.WriteString(`, `)
		}

		c.colWithTable(sel.Table, col.Col.Name)

		i++
	}
	for _, fn := range sel.Funcs {
		if i != 0 {
			c.w.WriteString(`, `)
		}
		c.renderFunction(sel, fn)
		i++
	}
}
