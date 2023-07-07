-- Add migration script herece
create table Categories_Posts(post_id integer, category_id integer,
 Foreign key (post_id) references Posts(id), foreign key (category_id) references Categories(id))