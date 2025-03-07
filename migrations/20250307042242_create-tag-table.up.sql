create table tags (
  id uuid primary key,
  title varchar(250) not null,
  description varchar(250),
  created_at timestamp not null default current_timestamp,
  updated_at timestamp not null default current_timestamp
);
