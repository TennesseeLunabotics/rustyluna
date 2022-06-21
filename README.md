# rustyluna

The Tennessee Lunabotics Open Robotics Stack written in Rust.

## Description

The purpose of this project is to provide a standardization base of control stacks for lunabotics competition robots. While other libraries and middleware suites exist (namely ROS), rustyluna aims to fit the needs of the NASA Robotic Mining Competition as well as adding safety and compatibility features due to being written in Rust. Additionally, due to competition constaints, the stack is built to use as little bandwidth as possible, and contain modular components to remove un-necessary functionality.

Most other robotics competitions such as Vex or FRC contain properitary hardware and software that makes the development of a robot easier. We understand that the lack of "boilerplate" material is to encourage innovation, but FOSS software that can be forked, improved upon, or outright redone does not necessarily impede on that. The MIT license encourages the perpetuation of the FOSS mindset, but does not implicitly state that forks have to be shared with community. 

## Getting Started

### Intended Hardware

Currently the stack is made to run on a combination of:


an arbitrary PC (client), 

Jetson Nano Developer Kit (server),

Arduino (serial).


Modules for additional types of input (gamepads, custom, etc) and sensors are not currently intended to be included in the base stack. These will have to be added or adapted into the codebase.


### Dependencies

rustc ❤️


libs via cargo: telnet, serialport, stick, regex

### Installing

Install rust.

Use cargo for libraries.

Compile and deploy server binary for Jetson Nano.

Compile and deploy client binary for PC.



## Authors

Adry Lain adrylain@gmail.com


## License

This project is licensed under the MIT License - see the LICENSE.md file for details
