DROP TABLE IF EXISTS constellation;
CREATE TABLE constellation (
    ID INTEGER PRIMARY KEY,
    Name VARCHAR(8) NOT NULL
);

DROP TABLE IF EXISTS stars;
CREATE TABLE stars (
    ID INTEGER PRIMARY KEY AUTOINCREMENT,
    Name VARCHAR(16) NOT NULL,
    CONSTELLATION INT NOT NULL,
    RA REAL NOT NULL,
    DECL REAL NOT NULL,
    Type TEXT CHECK(Type IN ('sun', 'twin', 'galaxy', 'cluster', 'nebula')) NOT NULL,
    MGA REAL
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
INSERT INTO stars(Name, CONSTELLATION, RA, DECL, Type, MGA)
VALUES ('南门二', 9, 14.728166667, -60.911916667, 'sun', -0.27);

INSERT INTO constellation VALUES (10, '宝瓶座');
INSERT INTO constellation VALUES (11, '蛇夫座');
INSERT INTO constellation VALUES (12, '狮子座');
INSERT INTO constellation VALUES (13, '牧夫座');
INSERT INTO constellation VALUES (14, '双鱼座');
INSERT INTO constellation VALUES (15, '人马座');
INSERT INTO constellation VALUES (16, '天鹅座');
INSERT INTO stars(Name, CONSTELLATION, RA, DECL, Type, MGA)
VALUES ('辇道增七', 16, 21.52761945, 28.01008333, 'twin', 3.35);
INSERT INTO stars(Name, CONSTELLATION, RA, DECL, Type, MGA)
VALUES ('辇道增七', 16, 21.52761945, 28.01008333, 'sun', 3.35);


INSERT INTO constellation VALUES (17, '金牛座');
INSERT INTO constellation VALUES (18, '鹿豹座');
INSERT INTO constellation VALUES (19, '仙女座');
INSERT INTO constellation VALUES (20, '船尾座');
INSERT INTO constellation VALUES (21, '御夫座');
INSERT INTO constellation VALUES (22, '天鹰座');
INSERT INTO constellation VALUES (23, '巨蛇座');
INSERT INTO constellation VALUES (24, '英仙座');
INSERT INTO stars(Name, CONSTELLATION, RA, DECL, Type, MGA)
VALUES ('英仙座h', 24, 2.3222222, 57.26944445, 'cluster', 3.8);

INSERT INTO constellation VALUES (25, '仙后座');
INSERT INTO constellation VALUES (26, '猎户座');
INSERT INTO constellation VALUES (27, '仙王座');
INSERT INTO constellation VALUES (28, '天猫座');
INSERT INTO constellation VALUES (29, '天秤座');
INSERT INTO constellation VALUES (30, '双子座');
INSERT INTO constellation VALUES (31, '巨蟹座');
INSERT INTO constellation VALUES (32, '船帆座');
INSERT INTO constellation VALUES (33, '天蝎座');
INSERT INTO constellation VALUES (34, '船底座');
INSERT INTO stars(Name, CONSTELLATION, RA, DECL, Type, MGA)
VALUES ('老人星', 34, 6.48611111, -52.76333333, 'sun', -0.65);
INSERT INTO constellation VALUES (35, '麒麟座');
INSERT INTO constellation VALUES (36, '玉夫座');
INSERT INTO constellation VALUES (37, '凤凰座');
INSERT INTO constellation VALUES (38, '猎犬座');
INSERT INTO constellation VALUES (39, '白羊座');
INSERT INTO constellation VALUES (40, '摩羯座');
INSERT INTO constellation VALUES (41, '天炉座');
INSERT INTO constellation VALUES (42, '后发座');

INSERT INTO constellation VALUES (43, '大犬座');
INSERT INTO stars(Name, CONSTELLATION, RA, DECL, Type, MGA)
VALUES ('天狼星', 43, 6.75250000, -16.71611111, 'sun', -1.45);
INSERT INTO constellation VALUES (44, '孔雀座');
INSERT INTO constellation VALUES (45, '天鹤座');
INSERT INTO constellation VALUES (46, '豺狼座');
INSERT INTO constellation VALUES (47, '六分仪座');
INSERT INTO constellation VALUES (48, '杜鹃座');
INSERT INTO constellation VALUES (49, '印第安座');
INSERT INTO constellation VALUES (50, '南极座');
INSERT INTO constellation VALUES (51, '天兔座');
INSERT INTO constellation VALUES (52, '天琴座');
INSERT INTO constellation VALUES (53, '巨爵座');
INSERT INTO constellation VALUES (54, '狐狸座');
INSERT INTO stars(Name, CONSTELLATION, RA, DECL, Type, MGA)
VALUES ('衣架星团', 54, 19.483333, 20.1833333, 'cluster', 3.6);
INSERT INTO constellation VALUES (55, '天鸽座');

INSERT INTO constellation VALUES (56, '小熊座');
INSERT INTO constellation VALUES (57, '望远镜座');
INSERT INTO constellation VALUES (58, '时钟座');
INSERT INTO constellation VALUES (59, '绘架座');
INSERT INTO constellation VALUES (60, '南鱼座');
INSERT INTO constellation VALUES (61, '水蛇座');
INSERT INTO constellation VALUES (62, '唧筒座');
INSERT INTO constellation VALUES (63, '天坛座');
INSERT INTO constellation VALUES (64, '小狮座');
INSERT INTO constellation VALUES (65, '罗盘座');
INSERT INTO constellation VALUES (66, '显微镜座');
INSERT INTO constellation VALUES (67, '天燕座');
INSERT INTO constellation VALUES (68, '蝎虎座');
INSERT INTO constellation VALUES (69, '海豚座');
INSERT INTO constellation VALUES (70, '乌鸦座');
INSERT INTO constellation VALUES (71, '小犬座');
INSERT INTO constellation VALUES (72, '北冕座');
INSERT INTO constellation VALUES (73, '剑鱼座');
INSERT INTO constellation VALUES (74, '矩尺座');
INSERT INTO constellation VALUES (75, '山案座');
INSERT INTO constellation VALUES (76, '飞鱼座');
INSERT INTO constellation VALUES (77, '苍蝇座');
INSERT INTO constellation VALUES (78, '蝘蜓座');
INSERT INTO constellation VALUES (79, '三角座');
INSERT INTO constellation VALUES (80, '南冕座');
INSERT INTO constellation VALUES (81, '雕具座');
INSERT INTO constellation VALUES (82, '网罟座');
INSERT INTO constellation VALUES (83, '南三角座');
INSERT INTO constellation VALUES (84, '盾牌座');
INSERT INTO constellation VALUES (85, '圆规座');
INSERT INTO constellation VALUES (86, '天箭座');
INSERT INTO constellation VALUES (87, '小马座');
INSERT INTO constellation VALUES (88, '南十字座');


