CREATE TYPE UserState AS ENUM ('startState', 'awaitingEventConfirmationState', 'confirmingCancellationState');

CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  first_name VARCHAR NOT NULL,
  last_name VARCHAR NOT NULL,
  phone_number VARCHAR NOT NULL,
  state UserState NOT NULL
);
