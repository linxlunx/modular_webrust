CREATE TABLE IF NOT EXISTS users (
        id int(100) AUTO_INCREMENT PRIMARY KEY,
        username varchar(255) NOT NULL,
        password varchar(255) NOT NULL,
        email varchar(255) NOT NULL,
        fullname varchar(255) NULL,
        created_at timestamp default NOW(),
        is_active tinyint default 1,
        is_deleted tinyint default 0,
        deleted_at timestamp default 0,
        deleted_by int(10) default 0
);

/* seeds data if empty, password "password" */
INSERT INTO users (username, password, email, fullname)
SELECT "linggar", "$2b$08$g/C2iGqL1QNIeIDBlKlAt.KZbueOR21GJLCceZz9OETCkJ/8U4vjG", "x@linggar.asia", "Linggar Primahastoko"
FROM DUAL
WHERE NOT EXISTS (SELECT * FROM users);