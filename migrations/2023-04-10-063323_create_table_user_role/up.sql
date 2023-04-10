CREATE TABLE IF NOT EXISTS user_role(
    id_user INT NOT NULL,
    id_role INT NOT NULL,
    KEY fk_role (id_role),
    KEY fk_user (id_user),
    PRIMARY KEY (id_user, id_role)
);