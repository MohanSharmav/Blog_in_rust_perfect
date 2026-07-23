create table categories (
    id integer primary key autoincrement,
    name varchar(225) not null,
    created_at timestamp not null default current_timestamp
);

create table posts (
    id integer primary key autoincrement,
    title varchar(225) not null,
    description varchar(224) not null,
    created_at timestamp not null default current_timestamp
);

create table categories_posts (
    post_id integer primary key,
    category_id integer not null,
    foreign key (post_id) references posts(id),
    foreign key (category_id) references categories(id)
);

create table users (
    name varchar(225) not null,
    password varchar(225) not null
);
