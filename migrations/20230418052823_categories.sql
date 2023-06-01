-- Add migration script here
create table categories(category_id SERIAL NOT NULL ,name varchar(255),PRIMARY KEY (name));