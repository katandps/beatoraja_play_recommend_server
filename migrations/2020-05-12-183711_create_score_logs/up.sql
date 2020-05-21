-- Your SQL goes here
create table score_logs
(
    id      int primary key,
    user_id int,
    data    mediumtext,
    foreign key fk_user_id (user_id) references users (id)
);