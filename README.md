# 推荐天体命令行软件
(Under development)

## 用法

使用Sqlite构建数据库：`sqlite3 stars.db < stars.sql`，

运行命令行程序：`recommend-stars lon lat ways type`，

其中
- `lon`：观测地经度，单位：度
- `lat`：观测地纬度，单位：度
- `ways`：观测方式，可选值：1-肉眼，1-肉眼，2-小型双筒，3-天文望远镜，4-深空摄影
- `type`：推荐类型，可选值：目标类型：sun, twin, galaxy, cluster, nebula

lon实际并没有被用到，可能会用于获取光污染等级

## 欢迎参与完善数据库！
由于作者能力有限，目前数据库中的天体仅有少量，诚邀各位天文爱好者参与完善数据库，格式如下：
```sqlite
INSERT INTO stars(Name, CONSTELLATION, RA, DECL, Type, MGA)
VALUES ('南门二', 9, 14.728166667, -60.911916667, 'sun', -0.27);
```
其中，Name为天体名称，CONSTELLATION为星座编号（见`.sql`文件），RA为赤经（24h制），DECL为赤纬，Type为天体类型（见上文`type`），MGA为（平均）视星等。

或者可以在issue中提供相应数据，作者看到后将会添加到数据库中。
## TODO
- ✅计算天体的在夜晚的最大高度以筛选今日的可观测目标
- ✅修改天体数据库格式，如取消id，转而用一个自增字段取代；以星座为界限添加天体目录
- ❓从其他软件中爬取亮星、梅西耶等数据，如Stellarium
- ❓完善天体数据库，参考Binocular Highlights、诺顿星图手册、猎户座左转等书籍
- ❓爬取光污染等级，调整极限星等
