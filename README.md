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
## TODO
- ✅计算天体的升落时间并筛选可观测目标
- ❓修改天体数据库格式，如取消id，转而用一个自增字段取代；以星座为界限添加天体目录
- ❓完善天体数据库，参考Binocular Highlights、诺顿星图手册、猎户座左转等书籍
- ❓爬取光污染等级，调整极限星等