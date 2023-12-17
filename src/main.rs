use crate::calc::visible_stars;

mod utils;
mod calc;

///
/// # Arguments
/// * `longitude` - 经度
/// * `latitude` - 纬度
/// * `ways` - 观测方式：1-肉眼，2-小型双筒，3-天文望远镜，4-深空摄影
/// * `obj_type` - 目标类型：sun, twin, galaxy, cluster, nebula
fn main() {
    let args: Vec<String> = std::env::args().collect();
    assert_eq!(args.len(), 5);
    let longitude = args[1].parse::<f64>().unwrap();
    let latitude = args[2].parse::<f64>().unwrap();
    let ways = args[3].parse::<usize>().unwrap();
    let obj_type = args[4].clone();
    visible_stars(longitude, latitude, ways, obj_type).unwrap();
}
