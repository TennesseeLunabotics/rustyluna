# rustyluna

The Tennessee Lunabotics Open Robotics Stack written in Rust.

## Description

The purpose of this project is to provide a base for standardization of control stacks for lunabotics competition robots. While other libraries and middleware suites exist (namely ROS), rustyluna aims to fit the needs of the NASA Robotic Mining Competition as well as adding safety and compatibility features due to being written in Rust. 

Most other robotics competitions such as Vex or FRC contain properitary hardware and software that makes the development of a robot easier. We understand that the lack of "boilerplate" material is to encourage innovation, but FOSS software that can be forked, improved upon, or outright redone does not necessarily impede on that. 

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



## Authors

Adry Lain adrylain@gmail.com


## License

This project is licensed under the MIT License - see the LICENSE.md file for details
