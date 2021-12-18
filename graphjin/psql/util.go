package psql

import (
	"bytes"
	"strconv"
)

func (c *compilerContext) alias(alias string) {
	c.w.WriteString(` AS `)
	c.quoted(alias)
}

func colWithTableID(w *bytes.Buffer, table string, id int32, col string) {
	w.WriteString(table)
	if id >= 0 {
		w.WriteString(`_`)
		int32String(w, id)
	}
	w.WriteString(`.`)
	w.WriteString(col)
}

func (c *compilerContext) colWithTable(table, col string) {
	c.quoted(table)
	c.w.WriteString(`.`)
	c.w.WriteString(col)
}

func (c *compilerContext) quoted(identifier string) {
	c.w.WriteByte('"')
	c.w.WriteString(identifier)
	c.w.WriteByte('"')
}

func (c *compilerContext) squoted(identifier string) {
	c.w.WriteByte('\'')
	c.w.WriteString(identifier)
	c.w.WriteByte('\'')
}

func int32String(w *bytes.Buffer, val int32) {
	w.WriteString(strconv.FormatInt(int64(val), 10))
}
