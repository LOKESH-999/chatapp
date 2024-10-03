CREATE TABLE users (
    user_id SERIAL PRIMARY KEY,
    username VARCHAR(70) NOT NULL,
    email_id VARCHAR(90) NOT NULL,
    dob DATE NOT NULL
);


CREATE TABLE auth (
    user_id SERIAL PRIMARY KEY,
    passwd VARCHAR(25) NOT NULL
);

CREATE TABLE chats (
    id BIGSERIAL PRIMARY KEY,
    from_ INT NOT NULL,
    to_ INT NOT NULL,
    msg VARCHAR(255) NOT NULL,
    sent TIMESTAMP NOT NULL,  
    received TIMESTAMP        
);
