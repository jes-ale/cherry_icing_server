CREATE TABLE IF NOT EXISTS catalog(
    id INT AUTO_INCREMENT,
    label VARCHAR(255) UNIQUE,
    note VARCHAR(255),
    PRIMARY KEY (id)
);