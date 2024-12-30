use std::sync::{Arc, Mutex};
fn main() {
    rosrust::init("sim_control_bridge");
    let rate = rosrust::rate(20.0);
    let cur_state = Arc::new(Mutex::new(msg::mavros_msgs::State::default()));
    let cur_state_clone = cur_state.clone();
    let _state_sub = rosrust::subscribe("/mavros/state", 10, move |state| {
        if let Ok(mut cur_state) = cur_state_clone.lock() {
            *cur_state = state;
        } else {
            rosrust::ros_err!("[STATE_SUB] Failed updating cur_state");
        }
    })
    .unwrap();

    let mut takeoff = msg::airsim::TakeoffReq::default();
    takeoff.waitOnLastTask = true;
    let take_off_cli: rosrust::Client<msg::airsim::Takeoff> =
        rosrust::client("airsim_node/drone_1/takeoff").unwrap();
    take_off_cli
        .req(&takeoff)
        .expect("Call take off srv failed")
        .expect("Take off failed");

    while rosrust::is_ok() && !cur_state.lock().unwrap().connected {
        rate.sleep();
    }

    let set_mode = msg::mavros_msgs::SetModeReq {
        base_mode: 0,
        custom_mode: "OFFBOARD".to_string(),
    };
    let arm = msg::mavros_msgs::CommandBoolReq { value: true };
    let mut last_request = rosrust::now();

    let arming_cli =
        rosrust::client::<msg::mavros_msgs::CommandBool>("/mavros/cmd/arming").unwrap();
    let set_mode_cli = rosrust::client::<msg::mavros_msgs::SetMode>("/mavros/set_mode").unwrap();

    while rosrust::is_ok() {
        if cur_state.lock().unwrap().mode != "OFFBOARD"
            && rosrust::now() - last_request > rosrust::Duration::from_seconds(5)
        {
            set_mode_cli
                .req(&set_mode)
                .expect("Call set mode srv failed")
                .expect("Set mode failed");
            last_request = rosrust::now();
        } else {
            if !cur_state.lock().unwrap().armed
                && rosrust::now() - last_request > rosrust::Duration::from_seconds(5)
            {
                arming_cli
                    .req(&arm)
                    .expect("Call arming srv failed")
                    .expect("Arming failed");
                last_request = rosrust::now();
            }
        }
        rate.sleep();
    }

    rosrust::spin();
}
