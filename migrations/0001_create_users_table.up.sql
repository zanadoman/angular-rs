CREATE TABLE "users" (
    "id"         bigserial PRIMARY KEY,
    "username"   varchar(50) NOT NULL,
    "password"   text NOT NULL,
    "created_at" timestamptz NOT NULL,
    "updated_at" timestamptz,
    "deleted_at" timestamptz
);
