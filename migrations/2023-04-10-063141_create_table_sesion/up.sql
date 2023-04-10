CREATE TABLE IF NOT EXISTS user_session(
    id INT AUTO_INCREMENT,
    id_user INT NOT NULL,
    jwt VARCHAR(255) UNIQUE NOT NULL,
    expiration TIMESTAMP NOT NULL,
    ip_address VARCHAR(11) NOT NULL,
    last_login TIMESTAMP NOT NULL,
    last_online TIMESTAMP NOT NULL,
    KEY fk_user (id_user),
    PRIMARY KEY (id)
);