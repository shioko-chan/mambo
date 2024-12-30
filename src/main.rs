use std::process::Command;

fn main() {
    let mut childs = vec![(
        "mavros",
        Command::new("roslaunch")
            .args([
                "mavros",
                "px4.launch",
                "fcu_url:=udp://:14540@127.0.0.1:14557",
            ])
            .spawn()
            .expect("Failed to start mavros"),
    )];
    childs.append(
        &mut ["sim_control_bridge", "sim_data_bridge"]
            .into_iter()
            .map(|crate_name| {
                (
                    crate_name,
                    Command::new("cargo")
                        .args(["run", "-p", crate_name])
                        .spawn()
                        .unwrap_or_else(|_| panic!("Failed to start child process {}", crate_name)),
                )
            })
            .collect(),
    );
    loop {
        if childs.iter_mut().any(|(name, child)| {
            child
                .try_wait()
                .expect(format!("Failed to check child process {}", name).as_str())
                .is_some()
        }) {
            break;
        }
    }
    childs.into_iter().for_each(|(name, mut child)| {
        child
            .kill()
            .expect(format!("Failed to kill child process {}", name).as_str());
    });
}
