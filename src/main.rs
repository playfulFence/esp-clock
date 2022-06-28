//#![feature(backtrace)]


// TODO add temp and humidity buffers to not refresh them to often

use std::ops::Add;
use std::{thread, time::*, ptr, string::String, str::*};

use std::result::Result::Ok;


use anyhow::*;
use embedded_graphics::geometry::AnchorPoint;
use embedded_hal::prelude::_embedded_hal_blocking_delay_DelayMs;
use log::*;

use esp_idf_hal::prelude::*;
use esp_idf_hal::*;
use esp_idf_sys::*;


use esp_idf_svc::systime::EspSystemTime;
use esp_idf_svc::timer::*;

use embedded_svc::sys_time::SystemTime;
use embedded_svc::timer::TimerService;
use embedded_svc::timer::*;

use time::OffsetDateTime;
use time::macros::offset;
use time::Date;


use embedded_graphics::mono_font::{ascii::FONT_10X20, MonoTextStyle};
use embedded_graphics::pixelcolor::*;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::*;
use embedded_graphics::text::*;
use embedded_text::alignment::* ;
use embedded_text::style::* ;
use embedded_text::TextBox;
use embedded_graphics::text::renderer::TextRenderer;

use ili9341::{self, Orientation};


use icm42670::{accelerometer::Accelerometer, Address, Icm42670};
use shared_bus::BusManagerSimple;
use shtcx::{shtc3, LowPower, PowerMode};

use display_interface_spi::SPIInterfaceNoCS;

use rustzx_core::zx::video::colors::ZXBrightness;
use rustzx_core::zx::video::colors::ZXColor;

mod display;
mod host;

const textStyle : TextStyle = TextStyleBuilder::new()
    .alignment(embedded_graphics::text::Alignment::Center)
    .baseline(embedded_graphics::text::Baseline::Middle)
    .build();

const MEASUREMENT_DELAY: i8 = 10;

fn main() -> Result<()> 
{
    esp_idf_sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    // Get backtraces from anyhow; only works for Xtensa arch currently
    #[cfg(arch = "xtensa")]
    env::set_var("RUST_BACKTRACE", "1");


    let peripherals = Peripherals::take().unwrap();
    let mut dp = display::create!(peripherals)?;

    let i2c = peripherals.i2c0;
    let sda = peripherals.pins.gpio10;
    let scl = peripherals.pins.gpio8;

    //let mut measurementDelay :embedded_hal::blocking::delay;


    &dp.clear(display::color_conv(ZXColor::White, ZXBrightness::Normal));


    unsafe{

    let config = <i2c::config::MasterConfig as Default>::default().baudrate(100.kHz().into());
    let mut i2c = i2c::Master::<i2c::I2C0, _, _>::new(i2c, i2c::MasterPins { sda, scl }, config)?;

    let bus = BusManagerSimple::new(i2c);
    let mut icm = Icm42670::new(bus.acquire_i2c(), Address::Primary).unwrap();
    let mut sht = shtc3(bus.acquire_i2c());

    sht.start_measurement(PowerMode::NormalMode);

        let mut tmInit : esp_idf_sys::tm = esp_idf_sys::tm{
            tm_sec: 14 as esp_idf_sys::c_types::c_int,
            tm_min: 29 as esp_idf_sys::c_types::c_int, 
            tm_hour: 16 as esp_idf_sys::c_types::c_int, 
            tm_mday: 10 as esp_idf_sys::c_types::c_int, 
            tm_mon: 5 as esp_idf_sys::c_types::c_int,  // starts with 0 
            tm_year: (2022  - 1900) as esp_idf_sys::c_types::c_int,
            tm_wday: 4 as esp_idf_sys::c_types::c_int,
            tm_yday: 161 as esp_idf_sys::c_types::c_int,
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
            
        let measurement = sht.get_measurement_result().unwrap();

        measurementsFlush(&mut dp,
                  &format!("{:+.0} C", measurement.temperature.as_degrees_celsius()), 
                   &format!("{:+.0} %RH", measurement.humidity.as_percent()),
                  display::color_conv);

        //let mut hum_buf = format!("{:+.0} %RH", measurement.humidity.as_percent());

        //info!("{:+.0} °C\t  {:+.0} %RH", measurement.temperature.as_degrees_celsius(), measurement.humidity.as_percent());

        let mut stupid_temp_counter = MEASUREMENT_DELAY; // temp and humidity will refresh once at minute (now at 10secs)
    
        loop
        {
             
            
            if (EspSystemTime{}.now().as_secs() as u64 != time_buf) {
                time_buf = EspSystemTime{}.now().as_secs() as u64;
                now = time as u64 + EspSystemTime{}.now().as_secs() as u64;

        
                stupid_temp_counter = stupid_temp_counter - 1;


                if (stupid_temp_counter == 0) {
                    sht.start_measurement(PowerMode::LowPower);  
                }

                info!("About to convert {} UNIX-timestamp to date-time fmt...", now);

                // let accel_norm = icm.accel_norm().unwrap();
                // let gyro_norm = icm.gyro_norm().unwrap();



                let mut rawTime = OffsetDateTime::from_unix_timestamp(now as i64)?;
                                    //.to_offset(offset!(+2));
                                    // .time()
                                    // .to_string();

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

                    info!("About to refresh temperature and humidity.");

                    let measurement = sht.get_measurement_result().unwrap();

                    info!(
                            "TEMP  = {:+.2} °C\t",
                            measurement.temperature.as_degrees_celsius() as i32
                        );
                    info!("RH   = {:+.2} %RH", measurement.humidity.as_percent());
                        
                    
                    let actual_temp = format!("{:+.0} C", measurement.temperature.as_degrees_celsius() as i32);
                    let actual_hum = format!("{:+.0} %RH", measurement.humidity.as_percent());
        
                
                    measurementsFlush(&mut dp,
                            &actual_temp, 
                             &actual_hum,
                            display::color_conv);

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

    Rectangle::with_center(display.bounding_box().center(), Size::new(120, 40))
        .into_styled(
            PrimitiveStyleBuilder::new()
                .fill_color(color_conv(ZXColor::White, ZXBrightness::Normal))
                .stroke_color(color_conv(ZXColor::Blue, ZXBrightness::Normal))
                .stroke_width(1)
                .build(),
        )
    .draw(display);



    Text::with_text_style(
        &toPrint,
        display.bounding_box().center(), //(display.bounding_box().size.height - 10) as i32 / 2),
        MonoTextStyle::new(&FONT_10X20, color_conv(ZXColor::Black, ZXBrightness::Normal)),
        textStyle,
    )
    .draw(display);

    Ok(())
} 


fn dateFlush<D>(display: &mut D, toPrint: &String, color_conv: fn(ZXColor, ZXBrightness) -> D::Color) -> anyhow::Result<()>
where
    D: DrawTarget + Dimensions,
{
    
    Rectangle::new(Point::zero(), Size::new(130, 30))
        .into_styled(
            PrimitiveStyleBuilder::new()
                .fill_color(color_conv(ZXColor::White, ZXBrightness::Normal))       /* for date in top-left of screen*/
                .stroke_color(color_conv(ZXColor::Blue, ZXBrightness::Normal))
                .stroke_width(1)
                .build(),
        )
    .draw(display);


    Text::with_alignment(
        &toPrint,
        Point::new(5,20), //(display.bounding_box().size.height - 10) as i32 / 2),
        MonoTextStyle::new(&FONT_10X20, color_conv(ZXColor::Black, ZXBrightness::Normal)),
        Alignment::Left)
    .draw(display);


    Ok(())
} 



fn weekdayFlush<D>(display: &mut D, toPrint: &String, color_conv: fn(ZXColor, ZXBrightness) -> D::Color) -> anyhow::Result<()>
where
    D: DrawTarget + Dimensions,
{
    
    Rectangle::with_center(display.bounding_box().center() - Size::new(0, 30), Size::new(120, 20))
        .into_styled(
            PrimitiveStyleBuilder::new()
                .fill_color(color_conv(ZXColor::White, ZXBrightness::Normal))
                .stroke_color(color_conv(ZXColor::Blue, ZXBrightness::Normal))
                .stroke_width(1)
                .build(),
        )
        .draw(display);


    Text::with_text_style(
        &toPrint,
        display.bounding_box().center() - Size::new(0, 30), //(display.bounding_box().size.height - 10) as i32 / 2),
        MonoTextStyle::new(&FONT_10X20, color_conv(ZXColor::Black, ZXBrightness::Normal)),
        textStyle,
    )
    .draw(display);


    Ok(())
} 

fn measurementsFlush<D>(display : &mut D, toPrintTemp: &String, toPrintHum: &String,  color_conv: fn(ZXColor, ZXBrightness) -> D::Color) -> anyhow::Result<()>
where
    D: DrawTarget + Dimensions, 
{

    // temperature
    Rectangle::new(Point::new(display.bounding_box().size.width as i32 - 120, 0), Size::new(120, 40))
        .into_styled(
            PrimitiveStyleBuilder::new()
                .fill_color(color_conv(ZXColor::White, ZXBrightness::Normal))
                .stroke_color(color_conv(ZXColor::Blue, ZXBrightness::Normal))
                .stroke_width(1)
                .build(),
        )
        .draw(display);


    Text::with_text_style(
        &toPrintTemp,
        Point::new(display.bounding_box().size.width as i32 - 60, 20), //(display.bounding_box().size.height - 10) as i32 / 2),
        MonoTextStyle::new(&FONT_10X20, color_conv(ZXColor::Black, ZXBrightness::Normal)),
        textStyle,
    )
    .draw(display);

                // temporary solution till bitmap-font issue won't be solved
    Circle::new(Point::new(display.bounding_box().size.width as i32 - 50, 14), 5)
    .into_styled(PrimitiveStyle::with_stroke(color_conv(ZXColor::Black, ZXBrightness::Normal), 1))
    .draw(display);

    //humidity

    Rectangle::new(Point::new(display.bounding_box().size.width as i32 - 120, display.bounding_box().size.height as i32 - 40), Size::new(120, 40))
    .into_styled(
        PrimitiveStyleBuilder::new()
            .fill_color(color_conv(ZXColor::White, ZXBrightness::Normal))
            .stroke_color(color_conv(ZXColor::Blue, ZXBrightness::Normal))
            .stroke_width(1)
            .build(),
    )
    .draw(display);


Text::with_text_style(
    &toPrintHum,
    Point::new(display.bounding_box().size.width as i32 - 60, display.bounding_box().size.height as i32 - 20), //(display.bounding_box().size.height - 10) as i32 / 2),
    MonoTextStyle::new(&FONT_10X20, color_conv(ZXColor::Black, ZXBrightness::Normal)),
    textStyle,
)
.draw(display);


    Ok(())
}