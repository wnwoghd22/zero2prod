-- Add migration script here

INSERT INTO users (user_id, username, password_hash)
VALUES (
    'c495700f-435d-4560-b35c-f095d7da2874',
    'admin',
    '$argon2id$v=19$m=15000,t=2,p=1$4/DHDks4OeMTh06bB6gaAA$W5FsU6u6fixcHlM+p0GbdAZXzmRzAno0bj+52AtlzPQ'
);