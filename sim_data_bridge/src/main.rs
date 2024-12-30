use std::sync::{Arc, Mutex};

fn main() {
    rosrust::init("sim_data_bridge");
    let rate = rosrust::rate(20.0);
    let gps_pub = rosrust::publish("/mavros/vision_pose/pose", 1).unwrap();
    let _ = rosrust::subscribe(
        "/airsim_node/drone_1/gps",
        10,
        move |pose: msg::geometry_msgs::PoseStamped| {
            gps_pub.send(pose).unwrap_or_else(|err| {
                rosrust::ros_err!("[GPS_SUB] ERROR publishing gps: {:?}", err);
            });
        },
    )
    .unwrap();
    // let imu_pub = rosrust::publish("/mavros/visi", 1).unwrap();
    // let _ = rosrust::subscribe(
    //     "/airsim_node/drone_1/imu/imu",
    //     1,
    //     move |pose: msg::sensor_msgs::Imu| {
    //         imu_pub.send(pose).unwrap_or_else(|err| {
    //             rosrust::ros_err!("[GPS_SUB] ERROR publishing gps: {:?}", err);
    //         });
    //     },
    // )
    // .unwrap();
    rosrust::spin();
}
