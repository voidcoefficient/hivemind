create table tasks (
  id uuid primary key,
  title varchar(250) not null,
  description varchar(250),
  completed boolean not null default false,
  created_at timestamp not null default current_timestamp,
  updated_at timestamp not null default current_timestamp,
  completedd_at timestamp not null default current_timestamp
);
