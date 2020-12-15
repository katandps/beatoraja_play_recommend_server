create table hashes
(
    id int primary key auto_increment,
    md5 varchar(32) not null,
    sha256 varchar(64) not null,
    index md5_index(md5),
    index sha256_index(sha256)
)