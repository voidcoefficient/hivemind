create table if not exists tasks(
  id uuid primary key,
  title varchar(300) not null,
  description varchar(300),
  completed boolean not null default false,
  created_at timestamp not null default current_timestamp,
  updated_at timestamp not null default current_timestamp,
  completed_at timestamp
);
