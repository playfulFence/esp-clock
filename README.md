# esp-clock :crab:

This project was inspired by the [rusty-clock](https://github.com/TeXitoi/rusty-clock) project


The initial goal of the project was just to implement clocks with brand new ESP32-C3-RUST-BOARD, but over time, new ideas and more use cases began to appear.

Generally, project is created to demonstrate the capabilities of the new board in the Rust language, but now I've taken the direction to create a monitoring system based on the RUST-BOARD, so ch
so you have two possibilites how to run this project:

* On the RUST-BOARD itself, enabling the `esp32c3_rust_board_ili9341` feature, in `Cargo.toml` file.
On-board sensors will be used.
* Using another chip (list of availible configurations somewhere below), activating the corresponding feature in `Cargo.toml` file.

>You can learn more about RUST-BOARD [here](https://github.com/esp-rs/esp-rust-board)


<br>


>### **Important** : every configuration other than `esp32c3_rust_board_ili9341` requires Wi-Fi connection and uses MQTT messaging to revceive data from RUST-BOARD, which is measuring temperature and humidity
>### MQTT measurements sender is implemented [here](https://github.com/playfulFence/esp32-mqtt-publish) also by myself and of course it's also for RUST-BOARD since this project is dedicated to it


<br>

## Hardware

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


<a data-flickr-embed="true" href="https://www.flickr.com/photos/196173186@N08/52242796290/in/dateposted-public/" title="esp-clock_esp-rs-logo"><img src="https://live.staticflickr.com/65535/52242796290_71cf2364e2_o.png" width="550" height="543" alt="esp-clock_esp-rs-logo"></a>

<a data-flickr-embed="true" href="https://www.flickr.com/photos/196173186@N08/52242317203/in/dateposted-public/" title="esp-clock_working"><img src="https://live.staticflickr.com/65535/52242317203_4c06ea1c69_o.png" width="550" height="543" alt="esp-clock_working"></a>


>### [Corresponding Wokwi project](https://wokwi.com/projects/336529450034266706)

<br>

## Features

- Enable `esp32c3_rust_board_ili9341` feature if you want to use RUST-BOARD and it's on-board sensors with `ili9341` display
- Enable `esp32c3_ili9341` if you're using `ESP32-C3` board and `ili9341` display **OR** if you want to use RUST-BOARD with **MQTT-messaging** to receive temperature and humidity from another RUST-BOARD
- Enable `esp32s3_ili9341` if you're using `ESP32-S3` board and `ili9341` display
- Enable `esp32s2_ili9341` if you're using `ESP32-S2` board and `ili9341` display
- Enable `esp32s3_usb_otg` if you're using `ESP32-S3-USB-OTG` board and `ili9341` display
> **Warning**
>
>  Not optimized yet.
- Enable `esp32s2_usb_otg` if you're using `ESP32-S2-USB-OTG` board and `ili9341` display
> **Warning**
>
>  Not optimized yet.

<br>

## Plans
- [ ] Model a box :alarm_clock:
- [ ] Make some kind of a monitoring system with the ability to place several sensors(RUST-BOARDS) over the room/building/whatever
- [ ] Optimize project for other targets (`ESP32-S3-BOX`, `ESP32-S2-Kaluga-1`, `ESP32-S2-HMI-DevKit-1`)
- [ ] [Grafana](https://grafana.com) integration :bar_chart:
- [ ] [Slint](https://slint-ui.com) integration :pager:

<br>

## Dev Containers
This repository offers Dev Containers supports for:
-  [Gitpod](https://gitpod.io/)
   - [![Open in Gitpod](https://gitpod.io/button/open-in-gitpod.svg)](https://gitpod.io/#https://github.com/playfulFence/esp-clock/tree/target/esp32s2/wokwi)
-  [VS Code Dev Containers](https://code.visualstudio.com/docs/remote/containers#_quick-start-open-an-existing-folder-in-a-container)
-  [GitHub Codespaces](https://docs.github.com/en/codespaces/developing-in-codespaces/creating-a-codespace)
> **Note**
>
> In order to use Gitpod the project needs to be published in a GitLab, GitHub,
> or Bitbucket repository.
>
> In [order to use GitHub Codespaces](https://github.com/features/codespaces#faq)
> the project needs to be published in a GitHub repository and the user needs
> to be part of the Codespaces beta or have the project under an organization.

If using VS Code or GitHub Codespaces, you can pull the image instead of building it
from the Dockerfile by selecting the `image` property instead of `build` in
`.devcontainer/devcontainer.json`. Further customization of the Dev Container can
be achived, see [.devcontainer.json reference](https://code.visualstudio.com/docs/remote/devcontainerjson-reference).

When using Dev Containers, some tooling to facilitate building, flashing and
simulating in Wokwi is also added.
### Build
- Terminal approach:

    ```
    scripts/build.sh  [debug | release]
    ```
    > If no argument is passed, `release` will be used as default


-  UI approach:

    The default build task is already set to build the project, and it can be used
    in VS Code and Gitpod:
    - From the [Command Palette](https://code.visualstudio.com/docs/getstarted/userinterface#_command-palette) (`Ctrl-Shift-P` or `Cmd-Shift-P`) run the `Tasks: Run Build Task` command.
    - `Terminal`-> `Run Build Task` in the menu.
    - With `Ctrl-Shift-B` or `Cmd-Shift-B`.
    - From the [Command Palette](https://code.visualstudio.com/docs/getstarted/userinterface#_command-palette) (`Ctrl-Shift-P` or `Cmd-Shift-P`) run the `Tasks: Run Task` command and
    select `Build`.
    - From UI: Press `Build` on the left side of the Status Bar.

### Flash

> **Note**
>
> When using GitHub Codespaces, we need to make the ports
> public, [see instructions](https://docs.github.com/en/codespaces/developing-in-codespaces/forwarding-ports-in-your-codespace#sharing-a-port).

- Terminal approach:
  - Using `flash.sh` script:

    ```
    scripts/flash.sh [debug | release]
    ```
    > If no argument is passed, `release` will be used as default

- UI approach:
    - From the [Command Palette](https://code.visualstudio.com/docs/getstarted/userinterface#_command-palette) (`Ctrl-Shift-P` or `Cmd-Shift-P`) run the `Tasks: Run Task` command and
    select `Build & Flash`.
    - From UI: Press `Build & Flash` on the left side of the Status Bar.
- Any alternative flashing method from host machine.


### Wokwi Simulation
When using a custom Wokwi project, please change the `WOKWI_PROJECT_ID` in
`run-wokwi.sh`. If no project id is specified, a DevKit for esp32c3 will be
used.
> **Warning**
>
>  ESP32-S3 is not available in Wokwi

- Terminal approach:

    ```
    scripts/run-wokwi.sh [debug | release]
    ```
    > If no argument is passed, `release` will be used as default

- UI approach:

    The default test task is already set to build the project, and it can be used
    in VS Code and Gitpod:
    - From the [Command Palette](https://code.visualstudio.com/docs/getstarted/userinterface#_command-palette) (`Ctrl-Shift-P` or `Cmd-Shift-P`) run the `Tasks: Run Test Task` command
    - With `Ctrl-Shift-,` or `Cmd-Shift-,`
        > **Note**
        >
        > This Shortcut is not available in Gitpod by default.
    - From the [Command Palette](https://code.visualstudio.com/docs/getstarted/userinterface#_command-palette) (`Ctrl-Shift-P` or `Cmd-Shift-P`) run the `Tasks: Run Task` command and
    select `Build & Run Wokwi`.
    - From UI: Press `Build & Run Wokwi` on the left side of the Status Bar.

> **Warning**
>
>  The simulation will pause if the browser tab is in the background.This may
> affect the execution, specially when debuging.

#### Debuging with Wokwi

Wokwi offers debugging with GDB.

- Terminal approach:
    ```
    $HOME/.espressif/tools/riscv32-esp-elf/esp-2021r2-patch3-8.4.0/riscv32-esp-elf/bin/riscv32-esp-elf-gdb target/riscv32imc-esp-espidf/debug/esp_clock -ex "target remote localhost:9333"
    ```

    > [Wokwi Blog: List of common GDB commands for debugging.](https://blog.wokwi.com/gdb-avr-arduino-cheatsheet/?utm_source=urish&utm_medium=blog)
- UI approach:
    1. Run the Wokwi Simulation in `debug` profile
    2. Go to `Run and Debug` section of the IDE (`Ctrl-Shift-D or Cmd-Shift-D`)
    3. Start Debugging by pressing the Play Button or pressing `F5`
    4. Choose the proper user:
        - `esp` when using VS Code or GitHub Codespaces
        - `gitpod` when using Gitpod
