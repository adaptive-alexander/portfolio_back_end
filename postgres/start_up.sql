CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE option_files
(
    id               uuid               default uuid_generate_v4() primary key,
    storage_location varchar   not null unique,
    created_at       timestamp not null default current_timestamp
)