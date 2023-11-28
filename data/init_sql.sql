-- Please first use cargo install sqlx-cli 
 -- Modify the database connection string in .env and config/config.toml 
 -- Then execute sqlx database create to create the database 
 -- Execute sqlx migrate run to restore the database, run the following SQL in the database to add default data. 
 -- After running, you can access /login with the default username:zhangsan and password:123.
BEGIN;
INSERT INTO "users" ("id", "username", "password") VALUES ('cdd0e080-5bb1-4442-b6f7-2ba60dbd0555', 'zhangsan', '$argon2id$v=19$m=19456,t=2,p=1$rcosL5pOPdA2c7i4ZuLA4Q$s0JGh78UzMmu1qZMpVUA3b8kWYLXcZhw7uBfwhYDJ4A');
COMMIT;
