DROP TABLE IF EXISTS constellation;
CREATE TABLE constellation (
    ID INT PRIMARY KEY,
    Name VARCHAR(8) NOT NULL
);

INSERT INTO constellation VALUES (1, '长蛇座');
INSERT INTO constellation VALUES (2, '室女座');
INSERT INTO constellation VALUES (3, '大熊座');
INSERT INTO constellation VALUES (4, '鲸鱼座');
INSERT INTO constellation VALUES (5, '武仙座');
INSERT INTO constellation VALUES (6, '波江座');
INSERT INTO constellation VALUES (7, '飞马座');
INSERT INTO constellation VALUES (8, '天龙座');
INSERT INTO constellation VALUES (9, '半人马座');
INSERT INTO constellation VALUES (10, '宝瓶座');
INSERT INTO constellation VALUES (11, '蛇夫座');
INSERT INTO constellation VALUES (12, '狮子座');
INSERT INTO constellation VALUES (13, '牧夫座');
INSERT INTO constellation VALUES (14, '双鱼座');
INSERT INTO constellation VALUES (15, '人马座');
INSERT INTO constellation VALUES (16, '天鹅座');
INSERT INTO constellation VALUES (17, '金牛座');
INSERT INTO constellation VALUES (18, '鹿豹座');
INSERT INTO constellation VALUES (19, '仙女座');
INSERT INTO constellation VALUES (20, '船尾座');
INSERT INTO constellation VALUES (21, '御夫座');
INSERT INTO constellation VALUES (22, '天鹰座');
INSERT INTO constellation VALUES (23, '巨蛇座');
INSERT INTO constellation VALUES (24, '英仙座');
INSERT INTO constellation VALUES (43, '大犬座');
INSERT INTO constellation VALUES (55, '狐狸座');

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


INSERT INTO stars VALUES (1, '天狼星', 43, 6.75250000, -16.71611111, 'sun', -1.46, NULL, NULL);
INSERT INTO stars VALUES (2, '老人星', 6.48611111, -52.76333333, 'sun', NULL, NULL);
INSERT INTO stars VALUES (111, '衣架星团', 55, 19.483333, 20.1833333, 'cluster', 3.6, NULL, NULL);
INSERT INTO stars VALUES (112, '英仙座h', 24, 2.3222222, 57.26944445, 'cluster', 3.8, NULL, NULL);