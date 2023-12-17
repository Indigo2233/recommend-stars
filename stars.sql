DROP TABLE IF EXISTS stars;

CREATE TABLE stars (
    ID INT PRIMARY KEY,
    Name VARCHAR(16) NOT NULL,
    CONSTELLATION INT NOT NULL,
    RA REAL NOT NULL,
    DECL REAL NOT NULL,
    Type TEXT CHECK(Type IN ('sun', 'twin', 'galaxy', 'cluster', 'nebula')) NOT NULL,
    MGA REAL,
    TWIN_ID INT,
    SPE_ANGLE INT
);

DROP TABLE IF EXISTS constellation;
CREATE TABLE constellation (
    ID INT PRIMARY KEY,
    Name VARCHAR(8) NOT NULL
);

INSERT INTO constellation VALUES (43, '大犬座');
INSERT INTO constellation VALUES (55, '狐狸座');

INSERT INTO stars VALUES (1, '天狼星', 43, 6.75250000, -16.71611111, 'sun', -1.46, NULL, NULL);
-- INSERT INTO stars VALUES (2, '老人星', 6.48611111, -52.76333333, 'sun', NULL, NULL);
INSERT INTO stars VALUES (111, '衣架星团', 55, 19.483333, 20.1833333, 'cluster', 3.6, NULL, NULL);