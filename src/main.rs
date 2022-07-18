//#![feature(backtrace)]


// TODO add temp and humidity buffers to not& refresh them to often

use std::ops::Add;
use std::sync::mpsc::channel;
use std::{thread, time::*, ptr, string::String};
use std::sync::Arc;

use std::result::Result::Ok;

extern crate cfg_if;

use anyhow::*;
use embedded_graphics::geometry::AnchorPoint;
use embedded_hal::prelude::_embedded_hal_blocking_delay_DelayMs;
use embedded_svc::httpd::app;
use log::*;

use esp_idf_hal::prelude::*;
use esp_idf_hal::*;
use esp_idf_sys::*;


use esp_idf_svc::systime::EspSystemTime;
use esp_idf_svc::timer::*;

use embedded_svc::sys_time::SystemTime;
use embedded_svc::timer::TimerService;
use embedded_svc::timer::*;

use time::{OffsetDateTime, format_description};
use time::macros::offset;
use time::Date;


use embedded_graphics::mono_font::{ MonoTextStyle };
use embedded_graphics::pixelcolor::*;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::*;
use embedded_graphics::text::*;
use embedded_graphics::image::Image;

// Font and image
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

// MQTT
use esp_idf_svc::{
    log::EspLogger,
    mqtt::client::*,
};
use embedded_svc::mqtt::client::{Client, Connection, MessageImpl, Publish, QoS, Event::*, Message};
use std::str;


// RustZX stuff
use rustzx_core::zx::video::colors::ZXBrightness;
use rustzx_core::zx::video::colors::ZXColor;

mod display;

const textStyle : TextStyle = TextStyleBuilder::new()
    .alignment(embedded_graphics::text::Alignment::Center)
    .baseline(embedded_graphics::text::Baseline::Middle)
    .build();

const MEASUREMENT_DELAY: i8 = 10;


#[toml_cfg::toml_config]
pub struct Config {
    #[default("")]
    mqtt_host: &'static str,
    #[default("")]
    mqtt_user: &'static str,
    #[default("")]
    wifi_ssid: &'static str,
    #[default("")]
    wifi_pass: &'static str,
}


fn main() -> Result<()> 
{
    esp_idf_sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let app_config = CONFIG;

    // Get backtraces from anyhow; only works for Xtensa arch currently
    #[cfg(arch = "xtensa")]
    env::set_var("RUST_BACKTRACE", "1");


    let peripherals = Peripherals::take().unwrap();
    let mut dp = display::create!(peripherals)?;

    show_logo(&mut dp);
    
   

    unsafe{

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

        let time : esp_idf_sys::time_t = esp_idf_sys::mktime(tmRef);
        
        let mut actual_date = OffsetDateTime::from_unix_timestamp(time as i64)?
                            .date();

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

        let mut temperature : &str;
        let mut humidity : &str;

        cfg_if::cfg_if! {
            if #[cfg(feature = "esp32c3_rust_board_ili9341")] {
                let i2c = peripherals.i2c0;
                let sda = peripherals.pins.gpio10;
                let scl = peripherals.pins.gpio8;

                let config = <i2c::config::MasterConfig as Default>::default().baudrate(100.kHz().into());
                let mut i2c = i2c::Master::<i2c::I2C0, _, _>::new(i2c, i2c::MasterPins { sda, scl }, config)?;
            
                let bus = BusManagerSimple::new(i2c);
                let mut icm = Icm42670::new(bus.acquire_i2c(), Address::Primary).unwrap();
                let mut sht = shtc3(bus.acquire_i2c());
            
                sht.start_measurement(PowerMode::LowPower);
            
                let measurement = sht.get_measurement_result().unwrap();

                temperature = &format!("{:+.0}°C", measurement.temperature.as_degrees_celsius() - 3.0);
                humidity = &format!("{:+.0}%RH", measurement.humidity.as_percent());

                measurementsFlush(&mut dp,
                        &format!("{:+.0}°C", measurement.temperature.as_degrees_celsius() - 3.0), 
                         &format!("{:+.0}%RH", measurement.humidity.as_percent()),
                        display::color_conv);
            }
            else {
                
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

                info!("About to set mqtt-configuration with client ID \"{}\"", app_config.mqtt_user);

                let mqtt_config = MqttClientConfiguration {
                    client_id: Some(app_config.mqtt_user),
                    ..Default::default()
                };
                
                let broker_url = "mqtt://broker.hivemq.com:1883";
                info!("About to connect mqtt-client");


                let (mut client, mut connection) = 
                        EspMqttClient::new_with_conn(app_config.mqtt_host, &mqtt_config)?;
                info!("Connected");

                let (sender, receiver) = channel();

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

                client.subscribe("esp-clock/measurements", QoS::AtLeastOnce)?;
                info!("Subscribed to topic \"esp-clock/measurements\"");

                let mut recv = receiver.recv().unwrap();


                info!("Received MQTT message in main thread: {}", recv);
                measurementsFlush(&mut dp,
                    &format!("{}°C", &recv[0..2]), 
                    &format!("{}%RH", &recv[5..8]), 
                    display::color_conv);
            }

        }


        let mut stupid_temp_counter = MEASUREMENT_DELAY; // temp and humidity will refresh once at minute (now at 10secs)
    
        loop
        {
            if (EspSystemTime{}.now().as_secs() as u64 != time_buf) {
                time_buf = EspSystemTime{}.now().as_secs() as u64;
                now = time as u64 + EspSystemTime{}.now().as_secs() as u64;

                stupid_temp_counter = stupid_temp_counter - 1;

                info!("About to convert {} UNIX-timestamp to date-time fmt...", now);

                // let accel_norm = icm.accel_norm().unwrap();
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

                if (stupid_temp_counter == 0) {
                    cfg_if::cfg_if! {
                        if #[cfg(feature = "esp32c3_rust_board_ili9341")] {
                            info!("About to refresh temperature and humidity.");

                            sht.start_measurement(PowerMode::LowPower);  

                            let measurement = sht.get_measurement_result().unwrap();

                            info!(
                                    "TEMP  = {:+.2} °C\t",
                                    measurement.temperature.as_degrees_celsius() as i32
                                );
                            info!("RH   = {:+.2} %RH", measurement.humidity.as_percent());
                                
                            
                            let actual_temp = format!("{:+.0}°C", measurement.temperature.as_degrees_celsius() as i32 - 3); // magic constant -3, cause sensors shows temp which is 3 more, than real
                            let actual_hum = format!("{:+.0}%RH", measurement.humidity.as_percent());
                
                        
                            measurementsFlush(&mut dp,
                                    &actual_temp, 
                                    &actual_hum,
                                    display::color_conv);
                        }
                        else {
                            recv = receiver.recv().unwrap();
                            measurementsFlush(&mut dp,
                                &format!("{}°C",&recv[0..2]), 
                                &format!("{}%RH",&recv[5..8]), 
                                display::color_conv);
                        }
                    }

                        stupid_temp_counter = MEASUREMENT_DELAY;
                }
                
            }
           
        }

    }

    Ok(())
}

//#[allow(dead_code)]
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
    
    Rectangle::with_center(display.bounding_box().center() - Size::new(0, 20), Size::new(120, 30))
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

fn measurementsFlush<D>(display : &mut D, toPrintTemp: &String, toPrintHum: &String, color_conv: fn(ZXColor, ZXBrightness) -> D::Color) -> anyhow::Result<()>
where
    D: DrawTarget + Dimensions, 
{

    // temperature
    Rectangle::new(Point::new(display.bounding_box().size.width as i32 - 80, 0), Size::new(80, 40))
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


    Rectangle::new(Point::new(display.bounding_box().size.width as i32 - 120, display.bounding_box().size.height as i32 - 40), Size::new(120, 40))
    .into_styled(
        PrimitiveStyleBuilder::new()
            .fill_color(color_conv(ZXColor::White, ZXBrightness::Normal))
            .stroke_color(color_conv(ZXColor::White, ZXBrightness::Normal))
            .stroke_width(1)
            .build(),
    )
    .draw(display);


Text::with_text_style(
    &toPrintHum,
    Point::new(display.bounding_box().size.width as i32 - 50, display.bounding_box().size.height as i32 - 20), //(display.bounding_box().size.height - 10) as i32 / 2),
    MonoTextStyle::new(&PROFONT_18_POINT, color_conv(ZXColor::Black, ZXBrightness::Normal)),
    textStyle,
)
.draw(display);

    Ok(())
}


fn show_logo<D>(display : &mut D) -> anyhow::Result<()>
where
    D: DrawTarget<Color = embedded_graphics::pixelcolor::Rgb565> + Dimensions,
{

    info!("Welcome!");
   
    display.clear(display::color_conv(ZXColor::White, ZXBrightness::Normal));
    let bmp = Bmp::<Rgb565>::from_slice(include_bytes!("../assets/esp-rs-big.bmp")).unwrap();
    Image::new(
        &bmp, 
        display.bounding_box().center() - Size::new(100, 100),
        )
    .draw(display);

    thread::sleep(Duration::from_secs(5));

    display.clear(display::color_conv(ZXColor::White, ZXBrightness::Normal));
    let bmp = Bmp::<Rgb565>::from_slice(include_bytes!("../assets/esp-rs-small.bmp")).unwrap();
    Image::new(
        &bmp, 
        Point::new(0, display.bounding_box().size.height as i32 - 50),
        )
    .draw(display);


    Ok(())
}

#[allow(dead_code)]
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


// fn get_temp(s: &String) ->  {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }

//     s.len()
// }


//#[cfg(feature = "esp32c3_ili9341")]


// #[cfg(feature = "esp32c3_ili9341")]
// fn temp_sens_init(peripherals: Peripherals) -> 