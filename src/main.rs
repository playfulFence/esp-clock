//#![feature(backtrace)]


use std::{thread, time::*, ptr, string::String, str::*};

use std::result::Result::Ok;

use time::OffsetDateTime;
use time::macros::offset;
use time::Date;

use anyhow::*;
use log::*;

use esp_idf_hal::prelude::*;
use esp_idf_hal::*;
use esp_idf_sys::*;

use esp_idf_svc::systime::EspSystemTime;
use esp_idf_svc::timer::*;

use embedded_svc::sys_time::SystemTime;
use embedded_svc::timer::TimerService;
use embedded_svc::timer::*;

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
use display_interface_spi::SPIInterfaceNoCS;

use rustzx_core::zx::video::colors::ZXBrightness;
use rustzx_core::zx::video::colors::ZXColor;

mod display;
mod host;

const textStyle : TextStyle = TextStyleBuilder::new()
    .alignment(embedded_graphics::text::Alignment::Center)
    .baseline(embedded_graphics::text::Baseline::Middle)
    .build();



fn main() -> Result<()> 
{
    // unsafe{
    //     esp_idf_sys::esp_wifi_set_mode(esp_idf_sys::wifi_mode_t_WIFI_MODE_STA);
    //     esp_idf_sys::esp_wifi_stop();
    // }

    esp_idf_sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    // Get backtraces from anyhow; only works for Xtensa arch currently
    #[cfg(arch = "xtensa")]
    env::set_var("RUST_BACKTRACE", "1");


    let peripherals = Peripherals::take().unwrap();
    let mut dp = display::create!(peripherals)?;

    &dp.clear(display::color_conv(ZXColor::White, ZXBrightness::Normal));



    unsafe{

        let mut tmInit : esp_idf_sys::tm = esp_idf_sys::tm{
            tm_sec: 50 as esp_idf_sys::c_types::c_int,
            tm_min: 59 as esp_idf_sys::c_types::c_int, 
            tm_hour: 21 as esp_idf_sys::c_types::c_int, 
            tm_mday: 15 as esp_idf_sys::c_types::c_int, 
            tm_mon: 6 as esp_idf_sys::c_types::c_int,
            tm_year: (2022  - 1900) as esp_idf_sys::c_types::c_int,
            tm_wday: 5 as esp_idf_sys::c_types::c_int,
            tm_yday: 165 as esp_idf_sys::c_types::c_int,
            tm_isdst: 0 as esp_idf_sys::c_types::c_int,
        };

        let tmRef: &mut esp_idf_sys::tm = &mut tmInit;

        let time : esp_idf_sys::time_t = esp_idf_sys::mktime(tmRef);
        
        let mut actualDate = OffsetDateTime::from_unix_timestamp(time as i64)?
                            .to_offset(offset!(+2))
                            .date();

        let mut dateStr = format!("{}-{}-{}", actualDate.to_calendar_date().2, 
                                              actualDate.to_calendar_date().1,
                                              actualDate.to_calendar_date().0);
        

        let mut now: u64 = 0;
        let mut timeBuf: u64 = 0; 

        dateFlush(
            &mut dp,
            &dateStr,
            display::color_conv);
        
        weekdayFlush(
            &mut dp, 
            &actualDate.weekday().to_string(),
            display::color_conv);

        loop
        {
            if (EspSystemTime{}.now().as_secs() as u64 != timeBuf) {
                timeBuf = EspSystemTime{}.now().as_secs() as u64;
                now = time as u64 + EspSystemTime{}.now().as_secs() as u64;

                info!("About to convert {} UNIX-timestamp to date-time fmt...", now);

                let mut rawTime = OffsetDateTime::from_unix_timestamp(now as i64)?
                                    .to_offset(offset!(+2));
                                    // .time()
                                    // .to_string();
                timeFlush(
                    &mut dp, 
                    &rawTime.time().to_string()[0..(rawTime.time().to_string().len() - 2)].to_string(),
                    display::color_conv);


                if actualDate != rawTime.date() {
                    actualDate = rawTime.date();
                    dateStr = format!("{}-{}-{}", actualDate.to_calendar_date().2,
                                                  actualDate.to_calendar_date().1, 
                                                  actualDate.to_calendar_date().0);
                    dateFlush(
                        &mut dp,
                        &dateStr,
                        display::color_conv);

                    weekdayFlush(
                        &mut dp,
                        &actualDate.weekday().to_string(),
                        display::color_conv);
                    
                }
                
            } else {
                continue;
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

    // TextBox::with_textbox_style(
    //     &toPrint,
    //     Rectangle::with_center(display.bounding_box().center(), Size::new(120, 40)),
    //         MonoTextStyle::new(&FONT_10X20, color_conv(ZXColor::Black, ZXBrightness::Normal)),
    //         TextBoxStyleBuilder::new()
    //             .height_mode(HeightMode::FitToText)
    //             .alignment(HorizontalAlignment::Center)
    //             .vertical_alignment(VerticalAlignment::Middle)
    //             .paragraph_spacing(6)
    //             .build(),
    // )
    // .draw(display);



    Text::with_text_style(
        &toPrint,
        display.bounding_box().center(), //(display.bounding_box().size.height - 10) as i32 / 2),
        MonoTextStyle::new(&FONT_10X20, color_conv(ZXColor::Black, ZXBrightness::Normal)),
        textStyle,
    )
    .draw(display);

    info!("LED rendering done");

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

    info!("LED rendering done");

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

    info!("LED rendering done");

    Ok(())
} 