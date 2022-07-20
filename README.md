# esp-clock

## Description

This project was inspired by the similar project for STM32F103C8: 
https://github.com/TeXitoi/rusty-clock

The initial goal of the project was just to implement clocks with brand new ESP32-C3-RUST-BOARD, but over time, new ideas and more use cases began to appear.

Generally, project is created to demonstrate the capabilities of the new board in the Rust language,
so you have to possibilites how to run this project:

* On the RUST-BOARD itself, enabling the `esp32c3_rust_board_ili9341` feature, in `Cargo.toml` file.
On-board sensors will be used.
* Using another chip (list of availible configurations somewhere below), activating the corresponding feature in `Cargo.toml` file.


>You can learn more about RUST-BOARD [here](https://github.com/esp-rs/esp-rust-board)


<br>


>### **Important** : every configuration other than `esp32c3_rust_board_ili9341` requires Wi-Fi connection and uses MQTT messaging. 
>### MQTT measurements sender is implemented [here](https://github.com/playfulFence/esp32-mqtt-publish) also by myself and of course it's also for RUST-BOARD since this project is dedicated to it

<br>

## Hardware
---
### ESP32-C3-RUST-BOARD
The basic use case for this project involves the use of a RUST-BOARD and [ILI9341](https://cdn-shop.adafruit.com/datasheets/ILI9341.pdf) display.

### Used pins
| ILI9341 |    ESP-RUST-BOARD   |
----------|---------------------|
| RST     | GPIO3               |
| CLK     | GPIO6               |
| D_C     | GPIO21              |
| CS      | GPIO20              |
| MOSI    | GPIO7               |
<br>

### ESP32-С3-RUST-BOARD with ILI9341 display

<a data-flickr-embed="true" href="https://www.flickr.com/photos/196173186@N08/52229608944/in/dateposted-public/" title="ESP-RUST-BOARD and ILI9341 connected"><img src="https://live.staticflickr.com/65535/52229608944_96a2c58072_o.png" width="500" height="477" alt="ESP-RUST-BOARD and ILI9341 connected"></a>


>### [Corresponding Wokwi project](https://wokwi.com/projects/336529450034266706)

<br>

## Features

- Enable `esp32c3_rust_board_ili9341` feature if you want to use RUST-BOARD and it's on-board sensors 
- Enable `esp32c3_ili9341` if you're using board with `C3` chip (including RUST-BOARD) **OR** if you want to use  **MQTT** to receive temperature and humidity 

to be continued... (*JoJo final song*)

