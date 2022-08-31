
use std::sync::mpsc::channel;
use std::{thread, time::*, ptr, string::String};
use std::str;
use std::result::Result::Ok;

extern crate cfg_if;
use cfg_if::cfg_if;

use anyhow::*;
use log::*;

// Common IDF stuff
use esp_idf_hal::prelude::*;
use esp_idf_hal::*;
use esp_idf_sys::*;

// Time stuff
use embedded_svc::sys_time::SystemTime;
use esp_idf_svc::systime::EspSystemTime;

use time::OffsetDateTime;
use time::macros::offset;

use esp_idf_svc::sntp;
use esp_idf_svc::sntp::SyncStatus;

// Graphic part
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::pixelcolor::*;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::*;
use embedded_graphics::text::*;
use embedded_graphics::image::Image;

// Fonts and image
use profont::{PROFONT_24_POINT, PROFONT_18_POINT};
use tinybmp::Bmp;

// Sensors
use icm42670::{accelerometer::Accelerometer, Address, Icm42670};
use shared_bus::BusManagerSimple;
use shtcx::{shtc3, LowPower, PowerMode, ShtC3};

// Wi-Fi
use embedded_svc::wifi::*;
use esp_idf_svc::netif::EspNetifStack;
use esp_idf_svc::nvs::EspDefaultNvs;
use esp_idf_svc::sysloop::EspSysLoopStack;
use esp_idf_svc::wifi::EspWifi;
use std::sync::Arc;

// MQTT
use esp_idf_svc::{
    log::EspLogger,
    mqtt::client::*,
};
use embedded_svc::mqtt::client::{Client, Connection, MessageImpl, Publish, QoS, Event::*, Message};

// RustZX spectrum stuff 
use rustzx_core::zx::video::colors::ZXBrightness;
use rustzx_core::zx::video::colors::ZXColor;

mod display;

const textStyle : TextStyle = TextStyleBuilder::new()
    .alignment(embedded_graphics::text::Alignment::Center)
    .baseline(embedded_graphics::text::Baseline::Middle)
    .build();




#[toml_cfg::toml_config]
pub struct Config {
    #[default("")]
    mqtt_user: &'static str,
    #[default("")]
    broker_url: &'static str,
    #[default("")]
    wifi_ssid: &'static str,
    #[default("")]
    wifi_pass: &'static str,
    #[default("measurements")]
    topic_name: &'static str,
}

const MEASUREMENT_DELAY: i32 = 10;

fn main() -> Result<()> 
{
    esp_idf_sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let app_config = CONFIG;

    // Get backtraces from anyhow; only works for Xtensa arch currently
    #[cfg(arch = "xtensa")]
    env::set_var("RUST_BACKTRACE", "1");

    // Set up peripherals and display
    let peripherals = Peripherals::take().unwrap();
    let mut dp = display::create!(peripherals)?;

    show_logo(&mut dp);
    wifi_image(&mut dp, false, display::color_conv);


    /*  If your configuration means use of WiFi :
            1) Every chip besides RUST-BOARD
            2) RUST-BOARD as a receiver (without using it's on-board sensors)
            3) Default config, but time is web-scrapped
        it will initialize it   */

    cfg_if::cfg_if! {
        if #[cfg(feature = "wifi")] {

            wifi_connecting(&mut dp, false, display::color_conv);

            /* Setup some stuff for WiFi initialization */
            let netif_stack = Arc::new(EspNetifStack::new()?);
            let sys_loop_stack = Arc::new(EspSysLoopStack::new()?);
            let default_nvs = Arc::new(EspDefaultNvs::new()?);
        
            info!("About to initialize WiFi (SSID: {}, PASS: {})", app_config.wifi_ssid, app_config.wifi_pass);

            let _wifi = wifi(
                netif_stack.clone(),
                sys_loop_stack.clone(),
                default_nvs.clone(),
                app_config.wifi_ssid,
                app_config.wifi_pass,
            )?;

            wifi_connecting(&mut dp, true, display::color_conv);

            
        }
    }
   
    /* Unsafe section is used since it's required, if you're using C functions and datatypes */
    unsafe{

        cfg_if::cfg_if! {
            if #[cfg(feature = "wifi")] {
                let sntp = sntp::EspSntp::new_default()?;
                info!("SNTP initialized, waiting for status!");

                while sntp.get_sync_status() != SyncStatus::Completed {}

                info!("SNTP status received!");

                let timer: *mut time_t = ptr::null_mut();
                
                let mut timestamp = esp_idf_sys::time(timer);

                let mut actual_date = OffsetDateTime::from_unix_timestamp(timestamp as i64)?
                                                .to_offset(offset!(+2))
                                                .date();

                info!("{} - {} - {}", actual_date.to_calendar_date().2, actual_date.to_calendar_date().1, actual_date.to_calendar_date().0);
     
            }
            else {
                /* If you choose non-wifi time method, you should set it manually */
                let mut tmInit : esp_idf_sys::tm = esp_idf_sys::tm{
                    tm_sec: 0 as esp_idf_sys::c_types::c_int,
                    tm_min: 52 as esp_idf_sys::c_types::c_int, 
                    tm_hour: 19 as esp_idf_sys::c_types::c_int, 
                    tm_mday: 28 as esp_idf_sys::c_types::c_int, 
                    tm_mon: 5 as esp_idf_sys::c_types::c_int,  // starts with 0 
                    tm_year: (2022  - 1900) as esp_idf_sys::c_types::c_int,
                    tm_wday: 4 as esp_idf_sys::c_types::c_int,
                    tm_yday: 179 as esp_idf_sys::c_types::c_int,
                    tm_isdst: 0 as esp_idf_sys::c_types::c_int,
                };

                let tmRef: &mut esp_idf_sys::tm = &mut tmInit;

                let timestamp : esp_idf_sys::time_t = esp_idf_sys::mktime(tmRef);
                
                let mut actual_date = OffsetDateTime::from_unix_timestamp(timestamp as i64)?
                                                .date();
            }
        }

        let mut date_str = format!("{}-{}-{}", actual_date.to_calendar_date().2, 
                                                       actual_date.to_calendar_date().1,
                                                       actual_date.to_calendar_date().0);
        

        let mut now: u64 = 0;
        let mut time_buf: u64 = 0; 

        dateFlush(
            &mut dp,
            &date_str,
            display::color_conv);
    
        weekdayFlush(
            &mut dp, 
            &actual_date.weekday().to_string(),
            display::color_conv);

        cfg_if::cfg_if! {
            if #[cfg(feature = "esp32c3_rust_board_ili9341")] {
                /* So, this feature means, that you're going to use on-board sensors of your RUST-BOARD */
                let i2c = peripherals.i2c0;
                let sda = peripherals.pins.gpio10;
                let scl = peripherals.pins.gpio8;

                let config = <i2c::config::MasterConfig as Default>::default().baudrate(100.kHz().into());
                let mut i2c = i2c::Master::<i2c::I2C0, _, _>::new(i2c, i2c::MasterPins { sda, scl }, config)?;
            
                let bus = BusManagerSimple::new(i2c);
                // let mut icm = Icm42670::new(bus.acquire_i2c(), Address::Primary).unwrap(); // TBD
                let mut sht = shtc3(bus.acquire_i2c());
                
                /* LowPower mode is used since NormalMode may cause 
                    code panicking if you're trying to take measurements "too early" */
                sht.start_measurement(PowerMode::LowPower);
                let measurement = sht.get_measurement_result().unwrap();

                measurementsFlush(
                        &mut dp,
                        &format!("{:+.0}°C", measurement.temperature.as_degrees_celsius() - 3.0), 
                        &format!("{:+.0}%RH", measurement.humidity.as_percent()),
                        true, 
                        display::color_conv);

                /* If your code is panicking here, consider using LowPower mode since NormalMode may cause 
                    code panicking if you're trying to take measurements "too early" */
                sht.start_measurement(PowerMode::NormalMode);
            }
            else {
                /* Otherwise, you will use MQTT messsaging and this (https://github.com/playfulFence/esp32-mqtt-publish) 
                   repo to make MQTT-measurements-sender from your another RUST-BOARD   */
                info!("About to set mqtt-configuration with client ID \"{}\"", app_config.mqtt_user);

                let mqtt_config = MqttClientConfiguration {
                    client_id: Some(app_config.mqtt_user),
                    ..Default::default()
                };
                
                info!("About to connect mqtt-client");


                let (mut client, mut connection) = 
                        EspMqttClient::new_with_conn(app_config.broker_url, &mqtt_config)?;
                info!("Connected");
                
                /* initialize pipe between threads */
                let (sender, receiver) = channel();

                // Need to immediately start pumping the connection for messages, or else subscribe() and publish() below will not work
                // Note that when using the alternative constructor - `EspMqttClient::new` - you don't need to
                // spawn a new thread, as the messages will be pumped with a backpressure into the callback you provide.
                // Yet, you still need to efficiently process each message in the callback without blocking for too long.
               

                thread::spawn(move || {
                    info!("MQTT Listening for messages");
                
                    while let Some(msg) = connection.next() {
                        match msg {
                            Err(e) => info!("MQTT Message ERROR: {}", e),
                            Ok(message) => {
                                match message {
                                    Received(recieved_bytes) => {
                                        match str::from_utf8(recieved_bytes.data()) {
                                            Err(e) => info!("MQTT Error : unreadable message! ({})", e),
                                            Ok(measurements) => sender.send(measurements.to_string()).unwrap(),
                                        }
                                    },
                                    BeforeConnect => info!("MQTT Message : Before connect"),
                                    Connected(tf) => info!("MQTT Message : Connected({})", tf),
                                    Disconnected => info!("MQTT Message : Disconnected"),
                                    Subscribed(message_id) => info!("MQTT Message : Subscribed({})", message_id),
                                    Unsubscribed(message_id) => info!("MQTT Message : Unsubscribed({})", message_id),
                                    Published(message_id) => info!("MQTT Message : Published({})", message_id),
                                    Deleted(message_id) => info!("MQTT Message : Deleted({})", message_id),
                                } 
                            },
                        }
                    }
            
                    info!("MQTT connection loop exit");
                });

                client.subscribe(app_config.topic_name, QoS::AtLeastOnce)?;
                info!("Subscribed to topic \"{}\"", app_config.topic_name);


                match receiver.try_recv() {
                    Err(e) => {
                        measurementsFlush(&mut dp,
                            &String::from("No\ndata"), 
                            &String::from("No\ndata"),
                            false,
                            display::color_conv);
                    },
                    Ok(response) => {
                        measurementsFlush(&mut dp,
                            &format!("{}°C",&response[0..2]), 
                            &format!("{}%RH",&response[5..8]),
                            true,
                            display::color_conv);
                    }
                }
            }
        }


        let mut stupid_temp_counter = MEASUREMENT_DELAY; // temp and humidity will refresh once at minute (now at 10secs)
    
        loop
        {
            cfg_if::cfg_if! {
                if #[cfg(not(feature = "wifi"))] {
                    now = timestamp as u64 + EspSystemTime{}.now().as_secs() as u64;

                    stupid_temp_counter = stupid_temp_counter - 1;

                    info!("About to convert {} UNIX-timestamp to date-time fmt...", now);

                    // let accel_norm = icm.accel_norm().unwrap(); // TBD
                    // let gyro_norm = icm.gyro_norm().unwrap();

                    let mut rawTime = OffsetDateTime::from_unix_timestamp(now as i64)?;

                    timeFlush(
                        &mut dp, 
                        &rawTime.time().to_string()[0..(rawTime.time().to_string().len() - 2)].to_string(),
                        display::color_conv);


                    if actual_date != rawTime.date() {

                        actual_date = rawTime.date();
                        date_str = format!("{}-{}-{}", actual_date.to_calendar_date().2,
                                                       actual_date.to_calendar_date().1, 
                                                       actual_date.to_calendar_date().0);

                        dateFlush(
                            &mut dp,
                            &date_str,
                            display::color_conv);

                        weekdayFlush(
                            &mut dp,
                            &actual_date.weekday().to_string(),
                            display::color_conv);
                    }
                }
                else {
                    timestamp = esp_idf_sys::time(timer);

                    let mut rawTime = OffsetDateTime::from_unix_timestamp(timestamp as i64)?
                                                    .to_offset(offset!(+2));

                    stupid_temp_counter = stupid_temp_counter - 1;

                    timeFlush(
                        &mut dp, 
                        &rawTime.time().to_string()[0..(rawTime.time().to_string().len() - 2)].to_string(),
                        display::color_conv);

                        if actual_date != rawTime.date() {

                        actual_date = rawTime.date();
                        date_str = format!("{}-{}-{}", actual_date.to_calendar_date().2,
                                                       actual_date.to_calendar_date().1, 
                                                       actual_date.to_calendar_date().0);

                        dateFlush(
                            &mut dp,
                            &date_str,
                            display::color_conv);

                        weekdayFlush(
                            &mut dp,
                            &actual_date.weekday().to_string(),
                            display::color_conv);
                    }

                }
            }

                if (stupid_temp_counter == 0) {

                    cfg_if::cfg_if! {

                        if #[cfg(feature = "esp32c3_rust_board_ili9341")] {

                            let measurement = sht.get_measurement_result().unwrap();
                            //info!("About to refresh temperature and humidity.");                

                            info!(
                                    "TEMP  = {:+.2} °C\t",
                                    measurement.temperature.as_degrees_celsius() as i32
                                );

                            info!(
                                    "RH   = {:+.2} %RH", 
                                    measurement.humidity.as_percent()
                            );
                                
                            
                            let actual_temp = format!("{:+.0}°C", measurement.temperature.as_degrees_celsius() as i32 - 3); // magic constant -3, cause sensors shows temp which is 3 more, than real
                            let actual_hum = format!("{:+.0}%RH", measurement.humidity.as_percent());
                
                        
                            measurementsFlush(&mut dp,
                                    &actual_temp, 
                                    &actual_hum,
                                    true,
                                    display::color_conv);

                            /* If your code is panicking here, consider using LowPower mode since NormalMode may cause 
                                code panicking if you're trying to take measurements "too early" */
                            sht.start_measurement(PowerMode::NormalMode);  
                        }
                        else {

                            info!("Waiting for message from MQTT thread");

                            /*  Classic recv function may cause the thread blocking since it just turns it
                                in waiting mode and clocks are stopped then 
                                Clocks stopped => you're dead! Do u want it? Me to... */
                            match receiver.try_recv() {
                                Err(e) => {
                                    measurementsFlush(
                                        &mut dp,
                                        &String::from("No\ndata"), 
                                        &String::from("No\ndata"),
                                        false,
                                        display::color_conv);
                                },
                                Ok(response) => {
                                    measurementsFlush(
                                        &mut dp,
                                        &format!("{}°C",&response[0..2]), 
                                        &format!("{}%RH",&response[5..8]),
                                        true,
                                        display::color_conv);
                                }
                            }        
                        }
                    }

                    stupid_temp_counter = MEASUREMENT_DELAY;
                }   
            thread::sleep(Duration::from_secs(1));
        }
    } // unsafe section
    Ok(())
}


fn timeFlush<D>(display: &mut D, toPrint: &String, color_conv: fn(ZXColor, ZXBrightness) -> D::Color) -> anyhow::Result<()>
where
    D: DrawTarget + Dimensions,
{
    Rectangle::with_center(display.bounding_box().center() + Size::new(0, 15), Size::new(132, 40))
        .into_styled(
            PrimitiveStyleBuilder::new()
                .fill_color(color_conv(ZXColor::White, ZXBrightness::Normal))
                .stroke_color(color_conv(ZXColor::White, ZXBrightness::Normal))
                .stroke_width(1)
                .build(),
        )
    .draw(display);

    Text::with_text_style(
        &toPrint,
        display.bounding_box().center() + Size::new(0, 10), //(display.bounding_box().size.height - 10) as i32 / 2),
        MonoTextStyle::new(&PROFONT_24_POINT, color_conv(ZXColor::Black, ZXBrightness::Normal)),
        textStyle,
    )
    .draw(display);

    Ok(())
} 


fn dateFlush<D>(display: &mut D, toPrint: &String, color_conv: fn(ZXColor, ZXBrightness) -> D::Color) -> anyhow::Result<()>
where
    D: DrawTarget + Dimensions,
{
    Rectangle::new(Point::zero(), Size::new(170, 30))
        .into_styled(
            PrimitiveStyleBuilder::new()
                .fill_color(color_conv(ZXColor::White, ZXBrightness::Normal))       /* for date in top-left of screen*/
                .stroke_color(color_conv(ZXColor::White, ZXBrightness::Normal))
                .stroke_width(1)
                .build(),
        )
    .draw(display);

    Text::with_alignment(
        &toPrint,
        Point::new(5,20), //(display.bounding_box().size.height - 10) as i32 / 2),
        MonoTextStyle::new(&PROFONT_18_POINT, color_conv(ZXColor::Black, ZXBrightness::Normal)),
        Alignment::Left)
    .draw(display);

    Ok(())
} 



fn weekdayFlush<D>(display: &mut D, toPrint: &String, color_conv: fn(ZXColor, ZXBrightness) -> D::Color) -> anyhow::Result<()>
where
    D: DrawTarget + Dimensions,
{
    Rectangle::with_center(display.bounding_box().center() - Size::new(0, 20), Size::new(140, 30))
        .into_styled(
            PrimitiveStyleBuilder::new()
                .fill_color(color_conv(ZXColor::White, ZXBrightness::Normal))
                .stroke_color(color_conv(ZXColor::White, ZXBrightness::Normal))
                .stroke_width(1)
                .build(),
        )
        .draw(display);

    Text::with_text_style(
        &toPrint,
        display.bounding_box().center() - Size::new(0, 25), //(display.bounding_box().size.height - 10) as i32 / 2),
        MonoTextStyle::new(&PROFONT_24_POINT, color_conv(ZXColor::Black, ZXBrightness::Normal)),
        textStyle,
    )
    .draw(display);

    Ok(())
} 
                                                                        /* if this bool is true => print humidity */
                                                                             /* otherwise - print "no data" */
fn measurementsFlush<D>(display : &mut D, toPrintTemp: &String, toPrintHum: &String, humOrND: bool, color_conv: fn(ZXColor, ZXBrightness) -> D::Color) -> anyhow::Result<()>
where
    D: DrawTarget + Dimensions, 
{
    // temperature
    Rectangle::new(Point::new(display.bounding_box().size.width as i32 - 80, 0), Size::new(80, 45))
        .into_styled(
            PrimitiveStyleBuilder::new()
                .fill_color(color_conv(ZXColor::White, ZXBrightness::Normal))
                .stroke_color(color_conv(ZXColor::White, ZXBrightness::Normal))
                .stroke_width(1)
                .build(),
        )
        .draw(display);

    Text::with_text_style(
        &toPrintTemp,
        Point::new(display.bounding_box().size.width as i32 - 35, 13), //(display.bounding_box().size.height - 10) as i32 / 2),
        MonoTextStyle::new(&PROFONT_18_POINT, color_conv(ZXColor::Black, ZXBrightness::Normal)),
        textStyle,
    )
    .draw(display);


    // humidity 
    Rectangle::new(Point::new(display.bounding_box().size.width as i32 - 80, display.bounding_box().size.height as i32 - 50), Size::new(120, 40))
    .into_styled(
        PrimitiveStyleBuilder::new()
            .fill_color(color_conv(ZXColor::White, ZXBrightness::Normal))
            .stroke_color(color_conv(ZXColor::White, ZXBrightness::Normal))
            .stroke_width(1)
            .build(),
    )
    .draw(display);

    if humOrND {
            //print humidity
        Text::with_text_style(
            &toPrintHum,
            Point::new(display.bounding_box().size.width as i32 - 50, display.bounding_box().size.height as i32 - 20),
            MonoTextStyle::new(&PROFONT_18_POINT, color_conv(ZXColor::Black, ZXBrightness::Normal)),
            textStyle,
        )
        .draw(display);
    }
    else { 
            // print "No Data"
        Text::with_text_style(
            &toPrintHum,
            Point::new(display.bounding_box().size.width as i32 - 35, display.bounding_box().size.height as i32 - 40),
            MonoTextStyle::new(&PROFONT_18_POINT, color_conv(ZXColor::Black, ZXBrightness::Normal)),
            textStyle,
        )
        .draw(display);
    }

    Ok(())
}


fn show_logo<D>(display : &mut D) -> anyhow::Result<()>
where
    D: DrawTarget<Color = embedded_graphics::pixelcolor::Rgb565> + Dimensions,
{
    info!("Welcome!");

    /* big logo at first */
    display.clear(display::color_conv(ZXColor::White, ZXBrightness::Normal));
    let bmp = Bmp::<Rgb565>::from_slice(include_bytes!("../assets/esp-rs-big.bmp")).unwrap();
    Image::new(
        &bmp, 
        display.bounding_box().center() - Size::new(100, 100),
        )
    .draw(display);

    thread::sleep(Duration::from_secs(5));

    /* than small */
    display.clear(display::color_conv(ZXColor::White, ZXBrightness::Normal));
    let bmp = Bmp::<Rgb565>::from_slice(include_bytes!("../assets/esp-rs-small.bmp")).unwrap();
    Image::new(
        &bmp, 
        Point::new(0, display.bounding_box().size.height as i32 - 50),
        )
    .draw(display);

    Ok(())
}


fn wifi(
    netif_stack: Arc<EspNetifStack>,
    sys_loop_stack: Arc<EspSysLoopStack>,
    default_nvs: Arc<EspDefaultNvs>,
    wifi_ssid : &str,
    wifi_password :&str,
) -> anyhow::Result<Box<EspWifi>> {
    let mut wifi = Box::new(EspWifi::new(netif_stack, sys_loop_stack, default_nvs)?);

    wifi.set_configuration(&Configuration::Client(ClientConfiguration {
        ssid: wifi_ssid.into(),
        password: wifi_password.into(),
        auth_method: AuthMethod::None,
        ..Default::default()
    }))?;

    println!("Wifi configuration set, about to get status");

    wifi.wait_status_with_timeout(Duration::from_secs(20), |status| !status.is_transitional())
        .map_err(|e| anyhow::anyhow!("Unexpected Wifi status: {:?}", e))?;

    info!("to get status");
    let status = wifi.get_status();

    info!("got status)");
    if let Status(
        ClientStatus::Started(ClientConnectionStatus::Connected(ClientIpStatus::Done(
            _ip_settings,
        ))),
        _,
    ) = status
    {
        println!("Wifi connected");
    } else {
        bail!("Unexpected Wifi status: {:?}", status);
    }

    Ok(wifi)
}

                            /* if this bool is true => wifi connected */
fn wifi_connecting<D>(display: &mut D, connected: bool, color_conv: fn(ZXColor, ZXBrightness) -> D::Color) -> anyhow::Result<()>
where
    D: DrawTarget<Color = embedded_graphics::pixelcolor::Rgb565> + Dimensions,
{
    Rectangle::with_center(display.bounding_box().center(), Size::new(display.bounding_box().size.width, 80))
    .into_styled(
        PrimitiveStyleBuilder::new()
            .fill_color(color_conv(ZXColor::White, ZXBrightness::Normal))
            .stroke_color(color_conv(ZXColor::White, ZXBrightness::Normal))
            .stroke_width(1)
            .build(),
    )
    .draw(display);

    if connected {
        Text::with_text_style(
            "Wi-Fi connected",
            display.bounding_box().center() - Size::new(0, 25), //(display.bounding_box().size.height - 10) as i32 / 2),
            MonoTextStyle::new(&PROFONT_24_POINT, color_conv(ZXColor::Black, ZXBrightness::Normal)),
            textStyle,
        )
        .draw(display);

        wifi_image(display, true, color_conv);

        thread::sleep(Duration::from_secs(2));

        Rectangle::with_center(display.bounding_box().center(), Size::new(display.bounding_box().size.width, 80))
        .into_styled(
            PrimitiveStyleBuilder::new()
                .fill_color(color_conv(ZXColor::White, ZXBrightness::Normal))
                .stroke_color(color_conv(ZXColor::White, ZXBrightness::Normal))
                .stroke_width(1)
                .build(),
        )
        .draw(display);
    }
    else {
        Text::with_text_style(
            "Connecting Wi-Fi...",
            display.bounding_box().center() - Size::new(0, 25), //(display.bounding_box().size.height - 10) as i32 / 2),
            MonoTextStyle::new(&PROFONT_24_POINT, color_conv(ZXColor::Black, ZXBrightness::Normal)),
            textStyle,
        )
        .draw(display);
    }

    Ok(())
}

                    /* if this bool is true => draw "WiFi connected image" */
                        /* otherwise - overcrossed WiFi image */
fn wifi_image<D>(display: &mut D, wifi: bool, color_conv: fn(ZXColor, ZXBrightness) -> D::Color) -> anyhow::Result<()>
where
    D: DrawTarget<Color = embedded_graphics::pixelcolor::Rgb565> + Dimensions,
{
    if wifi {
        Rectangle::new(Point::new(50, display.bounding_box().size.height as i32 - 50), Size::new(50, 50))
        .into_styled(
        PrimitiveStyleBuilder::new()
            .fill_color(color_conv(ZXColor::White, ZXBrightness::Normal))
            .stroke_color(color_conv(ZXColor::White, ZXBrightness::Normal))
            .stroke_width(1)
            .build(),
        )
        .draw(display);
        let bmp = Bmp::<Rgb565>::from_slice(include_bytes!("../assets/wifi.bmp")).unwrap();
        Image::new(
            &bmp, 
            Point::new(53, display.bounding_box().size.height as i32 - 50),
            )
        .draw(display);
    }
    else {
        let bmp = Bmp::<Rgb565>::from_slice(include_bytes!("../assets/wifi_not_connected.bmp")).unwrap();
        Image::new(
            &bmp, 
            Point::new(53, display.bounding_box().size.height as i32 - 50),
            )
        .draw(display);
    }

    Ok(())
}
