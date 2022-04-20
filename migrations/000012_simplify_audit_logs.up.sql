ALTER TABLE trust.logs
DROP COLUMN country,
DROP COLUMN region,
DROP COLUMN city,
DROP COLUMN name,
DROP COLUMN version,
DROP COLUMN os,
DROP COLUMN os_version,
DROP COLUMN device,
DROP COLUMN mobile,
DROP COLUMN tablet,
DROP COLUMN desktop,
DROP COLUMN bot,
DROP COLUMN url;

ALTER TABLE trust.logs RENAME COLUMN string TO user_agent;