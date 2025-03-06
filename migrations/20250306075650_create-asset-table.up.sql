create table assets (
  id uuid primary key,
  title varchar(250) not null,
  description varchar(250),
  amount integer not null default 0,
  created_at timestamp not null default current_timestamp,
  updated_at timestamp not null default current_timestamp
);
