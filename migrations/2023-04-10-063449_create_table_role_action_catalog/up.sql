CREATE TABLE IF NOT EXISTS role_action_catalog(
    id_role INT NOT NULL,
    id_action INT NOT NULL,
    id_catalog INT NOT NULL,
    KEY fk_role(id_role),
    KEY fk_action (id_action),
    KEY fk_catalog (id_catalog),
    PRIMARY KEY (id_role, id_action, id_catalog)
);