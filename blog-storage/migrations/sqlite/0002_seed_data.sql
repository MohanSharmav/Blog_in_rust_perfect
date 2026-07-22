insert into categories(name) values ('sports');
insert into categories(name) values ('travel');
insert into categories(name) values ('cars');

insert into posts(title,description) values('football','popular sports');
insert into posts(title,description) values('golf','solo sports');
insert into posts(title,description) values('cricket','best sports');
insert into posts(title,description) values('soccer','famous sports');
insert into posts(title,description) values('tennis','solo sports');
insert into posts(title,description) values('F1','cars sports');

insert into categories_posts(post_id, category_id) values(1,1);
insert into categories_posts(post_id, category_id) values(2,1);
insert into categories_posts(post_id, category_id) values(3,1);
insert into categories_posts(post_id, category_id) values(4,1);

insert into users(name, password) values('user','7KJkRPJSebW4At1v3o0fcQ==');
