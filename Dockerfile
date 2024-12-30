FROM osrf/ros:noetic-desktop-full-focal

USER root

WORKDIR /mambo

RUN apt update && apt install -y python3-catkin-tools \
    ros-noetic-geographic-msgs ros-noetic-tf2-sensor-msgs ros-noetic-tf2-geometry-msgs ros-noetic-image-transport \
    ros-noetic-mavros ros-noetic-mavros-extras ros-noetic-mavros-msgs \ 
    net-tools curl git protobuf-compiler libeigen3-dev libopencv-dev

ENV ROS_DISTRO noetic

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

ENV PATH="~/.cargo/bin:${PATH}"

ENV XMAKE_ROOT=y

RUN curl -fSL https://xmake.io/shget.text | bash && source ~/.xmake/profile

RUN git clone https://github.com/PX4/PX4-Autopilot.git --recursive 

RUN bash ./PX4-Autopilot/Tools/setup/ubuntu.sh --no-sim-tools --no-nuttx

RUN curl -fSL https://raw.githubusercontent.com/mavlink/mavros/master/mavros/scripts/install_geographiclib_datasets.sh | bash

COPY . /mambo

CMD ["cargo", "run"]
# roslaunch mavros px4.launch fcu_url:="udp://:14540@127.0.0.1:14557"