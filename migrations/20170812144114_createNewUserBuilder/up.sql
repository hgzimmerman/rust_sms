CREATE TABLE new_user_builders (
  id SERIAL PRIMARY KEY,
  first_name VARCHAR,
  last_name VARCHAR,
  phone_number VARCHAR,
  builder_state Serial NOT NULL
)
