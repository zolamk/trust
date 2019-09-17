-- This file should undo anything in `up.sql`
drop table users;

drop function if exists update_user_modified_at;