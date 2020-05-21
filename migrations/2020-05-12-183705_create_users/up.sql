-- Your SQL goes here
create table users
(
    id          int primary key,
    player_name varchar(128),
    email       varchar(255),
    password    varchar(255)
);