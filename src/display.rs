#![allow(unused_imports)]

#[cfg(all(feature = "kaluga_ili9341", not(esp32s2)))]
compile_error!(
    "The `kaluga_ili9341` feature can only be built for the `xtensa-esp32s2-espidf` target."
);

#[cfg(all(feature = "kaluga_st7789", not(esp32s2)))]
compile_error!(
    "The `kaluga_st7789` feature can only be built for the `xtensa-esp32s2-espidf` target."
);

#[cfg(all(feature = "ttgo", not(esp32)))]
compile_error!("The `ttgo` feature can only be built for the `xtensa-esp32-espidf` target.");

#[cfg(all(feature = "heltec", not(esp32)))]
compile_error!("The `heltec` feature can only be built for the `xtensa-esp32-espidf` target.");

#[cfg(all(feature = "esp32s2_usb_otg", not(esp32s2)))]
compile_error!(
    "The `esp32s2_usb_otg` feature can only be built for the `xtensa-esp32s2-espidf` target."
);

#[cfg(all(feature = "esp32s2_ili9341", not(esp32s2)))]
compile_error!(
    "The `esp32s2_ili9341` feature can only be built for the `xtensa-esp32s3-espidf` target."
);

#[cfg(all(feature = "esp32s3_usb_otg", not(esp32s3)))]
compile_error!(
    "The `esp32s3_usb_otg` feature can only be built for the `xtensa-esp32s3-espidf` target."
);

#[cfg(all(feature = "esp32s3_ili9341", not(esp32s3)))]
compile_error!(
    "The `esp32s3_ili9341` feature can only be built for the `xtensa-esp32s3-espidf` target."
);



#[cfg(not(any(
    feature = "kaluga_ili9341",
    feature = "esp32_ili9341",
    feature = "esp32c3_ili9341",
    feature = "kaluga_st7789",
    feature = "ttgo",
    feature = "heltec",
    feature = "esp32s2_usb_otg",
    feature = "esp32s3_usb_otg",
    feature = "esp32s2_ili9341",
    feature = "esp32s3_ili9341",
    feature = "esp32c3_rust_board_ili9341"
)))]
compile_error!("You have to define exactly one board with a LED screen using one of the features `ttgo`, `kaluga_ili9341`, `kaluga_st7789`, `esp32s2_usb_otg`, `esp32s3_usb_otg` or `heltec`.");

use anyhow::*;
use log::*;

use esp_idf_hal::gpio;
use esp_idf_hal::prelude::*;
use esp_idf_hal::spi;
use esp_idf_hal::{delay, i2c};

use display_interface_spi::SPIInterfaceNoCS;

use embedded_graphics::pixelcolor::*;
use embedded_graphics::prelude::*;

use ili9341;


use rustzx_core::zx::video::colors::ZXBrightness;
use rustzx_core::zx::video::colors::ZXColor;

use ssd1306::mode::DisplayConfig;
use st7789;
use embedded_hal::digital::v2::OutputPin;

macro_rules! create {
    ($peripherals:expr) => {{
        #[cfg(feature = "ttgo")]
        let result = display::ttgo_create_display(
            $peripherals.pins.gpio4,
            $peripherals.pins.gpio16,
            $peripherals.pins.gpio23,
            $peripherals.spi2,
            $peripherals.pins.gpio18,
            $peripherals.pins.gpio19,
            $peripherals.pins.gpio5,
        );

        #[cfg(feature = "heltec")]
        let result = display::heltec_create_display(
            $peripherals.pins.gpio16,
            $peripherals.i2c0,
            $peripherals.pins.gpio4,
            $peripherals.pins.gpio15,
        );

        #[cfg(any(feature = "esp32s2_usb_otg", feature = "esp32s3_usb_otg"))]
        let result = display::esp32s2s3_usb_otg_create_display(
            $peripherals.pins.gpio9,
            $peripherals.pins.gpio4,
            $peripherals.pins.gpio8,
            $peripherals.spi3,
            $peripherals.pins.gpio6,
            $peripherals.pins.gpio7,
            $peripherals.pins.gpio5,
        );

        #[cfg(feature = "kaluga_ili9341")]
        let result = display::kaluga_create_display_ili9341(
            $peripherals.pins.gpio6,
            $peripherals.pins.gpio13,
            $peripherals.pins.gpio16,
            $peripherals.spi3,
            $peripherals.pins.gpio15,
            $peripherals.pins.gpio7,
            $peripherals.pins.gpio11,
        );

        #[cfg(feature = "esp32_ili9341")]
        let result = display::esp32_create_display_ili9341(
            $peripherals.pins.gpio4,
            $peripherals.pins.gpio3,
            $peripherals.pins.gpio10,
            $peripherals.spi2,
            $peripherals.pins.gpio6,
            $peripherals.pins.gpio7,
            $peripherals.pins.gpio2,
        );

        #[cfg(feature = "esp32s2_ili9341")]
        let result = display::esp32s2_create_display_ili9341(
            $peripherals.pins.gpio20,
            $peripherals.pins.gpio1,
            $peripherals.pins.gpio0,
            $peripherals.spi2,
            $peripherals.pins.gpio8,
            $peripherals.pins.gpio21,
            $peripherals.pins.gpio9,
        );

        #[cfg(feature = "esp32s3_ili9341")]
        let result = display::esp32s3_create_display_ili9341(
            $peripherals.pins.gpio7,
            $peripherals.pins.gpio35,
            $peripherals.pins.gpio4,
            $peripherals.spi2,
            $peripherals.pins.gpio5,
            $peripherals.pins.gpio6,
            $peripherals.pins.gpio0,
        );

        #[cfg(any(feature = "esp32c3_ili9341", feature = "esp32c3_rust_board_ili9341"))]
        let result = display::esp32c3_create_display_ili9341(
            $peripherals.pins.gpio0,
            $peripherals.pins.gpio21,
            $peripherals.pins.gpio3,
            $peripherals.spi2,
            $peripherals.pins.gpio6,
            $peripherals.pins.gpio7,
            $peripherals.pins.gpio20,
        );
        

        #[cfg(feature = "kaluga_st7789")]
        let result = display::kaluga_create_display_st7789(
            $peripherals.pins.gpio6,
            $peripherals.pins.gpio13,
            $peripherals.pins.gpio16,
            $peripherals.spi3,
            $peripherals.pins.gpio15,
            $peripherals.pins.gpio9,
            $peripherals.pins.gpio11,
        );

        result
    }};
}

pub(crate) use create;

#[cfg(feature = "ttgo")]
pub(crate) fn ttgo_create_display(
    backlight: gpio::Gpio4<gpio::Unknown>,
    dc: gpio::Gpio16<gpio::Unknown>,
    rst: gpio::Gpio23<gpio::Unknown>,
    spi: spi::SPI2,
    sclk: gpio::Gpio18<gpio::Unknown>,
    sdo: gpio::Gpio19<gpio::Unknown>,
    cs: gpio::Gpio5<gpio::Unknown>,
) -> Result<
    st7789::ST7789<
        SPIInterfaceNoCS<
            spi::Master<
                spi::SPI2,
                gpio::Gpio18<gpio::Unknown>,
                gpio::Gpio19<gpio::Unknown>,
                gpio::Gpio21<gpio::Unknown>,
                gpio::Gpio5<gpio::Unknown>,
            >,
            gpio::Gpio16<gpio::Output>,
        >,
        gpio::Gpio23<gpio::Output>,
    >,
> {
    info!("About to initialize the TTGO ST7789 LED driver");

    let config = <spi::config::Config as Default>::default()
        .baudrate(26.MHz().into())
        .bit_order(spi::config::BitOrder::MSBFirst);

    let mut backlight = backlight.into_output()?;
    backlight.set_high()?;

    let di = SPIInterfaceNoCS::new(
        spi::Master::<spi::SPI2, _, _, _, _>::new(
            spi,
            spi::Pins {
                sclk,
                sdo,
                sdi: Option::<gpio::Gpio21<gpio::Unknown>>::None,
                cs: Some(cs),
            },
            config,
        )?,
        dc.into_output()?,
    );

    let mut display = st7789::ST7789::new(
        di,
        rst.into_output()?,
        // SP7789V is designed to drive 240x320 screens, even though the TTGO physical screen is smaller
        240,
        320,
    );

    display.init(&mut delay::Ets);
    display
        .set_orientation(st7789::Orientation::Portrait);

    // The TTGO board's screen does not start at offset 0x0, and the physical size is 135x240, instead of 240x320
    /*let top_left = Point::new(52, 40);
    let size = Size::new(135, 240);

    Ok(
        display.cropped(&embedded_graphics::primitives::Rectangle::new(
            top_left, size,
        )),
    )*/

    Ok(display)
}

#[cfg(feature = "esp32_ili9341")]
pub(crate) fn esp32_create_display_ili9341(
    backlight: gpio::Gpio4,
    dc: gpio::Gpio3,
    rst: gpio::Gpio10,
    spi: spi::SPI2,
    sclk: gpio::Gpio6,
    sdo: gpio::Gpio7,
    cs: gpio::Gpio2,
) -> Result<
    ili9341::Ili9341<
        SPIInterfaceNoCS<
            spi::SpiDeviceDriver<'d, 
                spi::SpiDriver<'d>
            >, 
            gpio::PinDriver<'d,
                gpio::Gpio3, 
                gpio::Output
            >
        >,
        gpio::PinDriver<'d,
                gpio::Gpio10, 
                gpio::Output
            >,  
    >,
> {
    // Kaluga needs customized screen orientation commands
    // (not a surprise; quite a few ILI9341 boards need these as evidences in the TFT_eSPI & lvgl ESP32 C drivers)
    pub enum KalugaOrientation {
        Portrait,
        PortraitFlipped,
        Landscape,
        LandscapeFlipped,
    }

    impl ili9341::Mode for KalugaOrientation {
        fn mode(&self) -> u8 {
            match self {
                Self::Portrait => 0,
                Self::Landscape => 0x20 | 0x40,
                Self::PortraitFlipped => 0x80 | 0x40,
                Self::LandscapeFlipped => 0x80 | 0x20,
            }
        }

        fn is_landscape(&self) -> bool {
            matches!(self, Self::Landscape | Self::LandscapeFlipped)
        }
    }

    info!("About to initialize the ESP32C3 ILI9341 SPI LED driver");

    let config = <spi::config::Config as Default>::default()
        .baudrate(40.MHz().into());
        //.bit_order(spi::config::BitOrder::MSBFirst);

    let mut backlight = backlight.into_output()?;
    backlight.set_low()?;

    let mut backlight = pin::PinDriver::output(backlight);
    backlight.set_low()?;

    let di = SPIInterfaceNoCS::new(
        spi::SpiDeviceDriver::new_single(
            spi,
            sclk,
            sdo,
            Option::<gpio::AnyIOPin>::None,
            spi::Dma::Disabled,
            Some(cs),
            &spi::SpiConfig::new().baudrate(40.MHz().into()),
        )?,
        gpio::PinDriver::output(dc)?,
    );

    let reset = gpio::PinDriver::output(rst)?;

    ili9341::Ili9341::new(
        di,
        reset,
        &mut delay::Ets,
        KalugaOrientation::Landscape,
        // KalugaOrientation::LandscapeFlipped // uncomment this line and comment the line above for correct Wokwi simulation
        ili9341::DisplaySize240x320,
    ).map_err(|e| anyhow!("Failed to init display"))
}

#[cfg(feature = "esp32s2_ili9341")]
pub(crate) fn esp32s2_create_display_ili9341(
    backlight: gpio::Gpio20,
    dc: gpio::Gpio1, // 
    rst: gpio::Gpio0, //
    spi: spi::SPI2,
    sclk: gpio::Gpio8, //
    sdo: gpio::Gpio21,  //
    cs: gpio::Gpio9, // -> 20
) -> Result<
    ili9341::Ili9341<
        SPIInterfaceNoCS<
            spi::SpiDeviceDriver<'d, 
                spi::SpiDriver<'d>
            >,
            gpio::PinDriver<'d,
                gpio::Gpio1, 
                gpio::Output
            >,
        >,
            gpio::PinDriver<'d,
                gpio::Gpio0, 
                gpio::Output
            >,
    >,
> {
    // Kaluga needs customized screen orientation commands
    // (not a surprise; quite a few ILI9341 boards need these as evidences in the TFT_eSPI & lvgl ESP32 C drivers)
    // Display orientation: https://cdn-shop.adafruit.com/datasheets/ILI9341.pdf
    // Page 209
    pub enum KalugaOrientation {
        Portrait,
        PortraitFlipped,
        Landscape,
        LandscapeVericallyFlipped,
        LandscapeFlipped,
    }

    impl ili9341::Mode for KalugaOrientation {
        fn mode(&self) -> u8 {
            match self {
                Self::Portrait => 0,
                Self::LandscapeVericallyFlipped => 0x20,
                Self::Landscape => 0x20 | 0x40,
                Self::PortraitFlipped => 0x80 | 0x40,
                Self::LandscapeFlipped => 0x80 | 0x20,
            }
        }

        fn is_landscape(&self) -> bool {
            matches!(self, Self::Landscape | Self::LandscapeFlipped | Self::LandscapeVericallyFlipped)
        }
    }

    info!("About to initialize the ESP32C3 ILI9341 SPI LED driver");

    let config = <spi::config::Config as Default>::default()
        .baudrate(40.MHz().into());
        //.bit_order(spi::config::BitOrder::MSBFirst);

    let mut backlight = pin::PinDriver::output(backlight);
    backlight.set_low()?;

    let di = SPIInterfaceNoCS::new(
        spi::SpiDeviceDriver::new_single(
            spi,
            sclk,
            sdo,
            Option::<gpio::AnyIOPin>::None,
            spi::Dma::Disabled,
            Some(cs),
            &spi::SpiConfig::new().baudrate(40.MHz().into()),
        )?,
        gpio::PinDriver::output(dc)?,
    );

    let reset = gpio::PinDriver::output(rst)?;

    ili9341::Ili9341::new(
        di,
        reset,
        &mut delay::Ets,
        KalugaOrientation::Landscape,
        // KalugaOrientation::LandscapeFlipped // uncomment this line and comment the line above for correct Wokwi simulation
        ili9341::DisplaySize240x320,
    ).map_err(|e| anyhow!("Failed to init display"))
}


#[cfg(feature = "esp32s3_ili9341")]  // TODO
pub(crate) fn esp32s3_create_display_ili9341(
    backlight: gpio::Gpio7<gpio::Unknown>,
    dc: gpio::Gpio35, // 
    rst: gpio::Gpio4, //
    spi: spi::SPI2,
    sclk: gpio::Gpio5, //
    sdo: gpio::Gpio6,  //
    cs: gpio::Gpio0, // -> 20
) -> Result<
    ili9341::Ili9341<
        SPIInterfaceNoCS<
            spi::SpiDeviceDriver<'d, 
            spi::SpiDriver<'d>
                >, 
            gpio::PinDriver<'d,
                gpio::Gpio35, 
                gpio::Output
            >
        >,
        gpio::PinDriver<'d,
                gpio::Gpio4, 
                gpio::Output
        >,  
    >,
> {
    // Kaluga needs customized screen orientation commands
    // (not a surprise; quite a few ILI9341 boards need these as evidences in the TFT_eSPI & lvgl ESP32 C drivers)
    // Display orientation: https://cdn-shop.adafruit.com/datasheets/ILI9341.pdf
    // Page 209
    pub enum KalugaOrientation {
        Portrait,
        PortraitFlipped,
        Landscape,
        LandscapeVericallyFlipped,
        LandscapeFlipped,
    }

    impl ili9341::Mode for KalugaOrientation {
        fn mode(&self) -> u8 {
            match self {
                Self::Portrait => 0,
                Self::LandscapeVericallyFlipped => 0x20,
                Self::Landscape => 0x20 | 0x40,
                Self::PortraitFlipped => 0x80 | 0x40,
                Self::LandscapeFlipped => 0x80 | 0x20,
            }
        }

        fn is_landscape(&self) -> bool {
            matches!(self, Self::Landscape | Self::LandscapeFlipped | Self::LandscapeVericallyFlipped)
        }
    }

    info!("About to initialize the ESP32C3 ILI9341 SPI LED driver");

    let config = <spi::config::Config as Default>::default()
        .baudrate(40.MHz().into());
        //.bit_order(spi::config::BitOrder::MSBFirst);

    let mut backlight = pin::PinDriver::output(backlight);
    backlight.set_low()?;

    let di = SPIInterfaceNoCS::new(
        spi::SpiDeviceDriver::new_single(
            spi,
            sclk,
            sdo,
            Option::<gpio::AnyIOPin>::None,
            spi::Dma::Disabled,
            Some(cs),
            &spi::SpiConfig::new().baudrate(40.MHz().into()),
        )?,
        gpio::PinDriver::output(dc)?,
    );

    let reset = gpio::PinDriver::output(rst)?;

    ili9341::Ili9341::new(
        di,
        reset,
        &mut delay::Ets,
        KalugaOrientation::Landscape,
        // KalugaOrientation::LandscapeFlipped // uncomment this line and comment the line above for correct Wokwi simulation
        ili9341::DisplaySize240x320,
    ).map_err(|e| anyhow!("Failed to init display"))
}

#[cfg(any(feature = "esp32c3_ili9341", feature = "esp32c3_rust_board_ili9341"))]
pub(crate) fn esp32c3_create_display_ili9341<'d>(
    backlight: gpio::Gpio0,
    dc: gpio::Gpio21,  
    rst: gpio::Gpio3, 
    spi: spi::SPI2,
    sclk: gpio::Gpio6, 
    sdo: gpio::Gpio7,  
    cs: gpio::Gpio20, 
) -> Result<
    ili9341::Ili9341<
        SPIInterfaceNoCS<
            spi::SpiDeviceDriver<'d, 
                spi::SpiDriver<'d>
            >, 
            gpio::PinDriver<'d,
                gpio::Gpio21, 
                gpio::Output
            >
        >,
        gpio::PinDriver<'d,
                gpio::Gpio3, 
                gpio::Output
        >,  
    >,
> {
    use esp_idf_hal::{spi::SpiDeviceDriver, gpio::OutputPin};

    // Kaluga needs customized screen orientation commands
    // (not a surprise; quite a few ILI9341 boards need these as evidences in the TFT_eSPI & lvgl ESP32 C drivers)
    // Display orientation: https://cdn-shop.adafruit.com/datasheets/ILI9341.pdf
    // Page 209
    pub enum KalugaOrientation {
        Portrait,
        PortraitFlipped,
        Landscape,
        LandscapeVericallyFlipped,
        LandscapeFlipped,
    }

    impl ili9341::Mode for KalugaOrientation {
        fn mode(&self) -> u8 {
            match self {
                Self::Portrait => 0,
                Self::LandscapeVericallyFlipped => 0x20,
                Self::Landscape => 0x20 | 0x40,
                Self::PortraitFlipped => 0x80 | 0x40,
                /* this is used for Wokwi simulation, "| 0x08" is used to invert colors to correct */
                Self::LandscapeFlipped => 0x80 | 0x20 | 0x08, 
            }
        }

        fn is_landscape(&self) -> bool {
            matches!(self, Self::Landscape | Self::LandscapeFlipped | Self::LandscapeVericallyFlipped)
        }
    }

    info!("About to initialize the ESP32C3 ILI9341 SPI LED driver");
        //.bit_order(spi::config::BitOrder::MSBFirst);

    let mut backlight = gpio::PinDriver::output(backlight)?;
    backlight.set_low()?;
    

    let di = SPIInterfaceNoCS::new(
        spi::SpiDeviceDriver::new_single(
            spi,
            sclk,
            sdo,
            Option::<gpio::AnyIOPin>::None,
            spi::Dma::Disabled,
            Some(cs),
            &spi::SpiConfig::new().baudrate(40.MHz().into()),
        )?,
        gpio::PinDriver::output(dc)?,
    );

    let reset = gpio::PinDriver::output(rst)?;

    ili9341::Ili9341::new(
        di,
        reset,
        &mut delay::Ets,
        KalugaOrientation::Landscape,
        // KalugaOrientation::LandscapeFlipped // uncomment this line and comment the line above for correct Wokwi simulation
        ili9341::DisplaySize240x320,
    ).map_err(|e| anyhow!("Failed to init display"))
}


#[cfg(feature = "kaluga_ili9341")]
pub(crate) fn kaluga_create_display_ili9341(
    backlight: gpio::Gpio6,
    dc: gpio::Gpio13,
    rst: gpio::Gpio16,
    spi: spi::SPI3,
    sclk: gpio::Gpio15,
    sdo: gpio::Gpio9,
    cs: gpio::Gpio11,
) -> Result<
        ili9341::Ili9341<
            SPIInterfaceNoCS<
                spi::SpiDeviceDriver<'d, 
                    spi::SpiDriver<'d>
                >, 
                gpio::PinDriver<'d,
                    gpio::Gpio21, 
                    gpio::Output
                >
            >,
            gpio::PinDriver<'d,
                    gpio::Gpio3, 
                    gpio::Output
            >,  
        >,
> {
    // Kaluga needs customized screen orientation commands
    // (not a surprise; quite a few ILI9341 boards need these as evidences in the TFT_eSPI & lvgl ESP32 C drivers)
    pub enum KalugaOrientation {
        Portrait,
        PortraitFlipped,
        Landscape,
        LandscapeFlipped,
    }

    impl ili9341::Mode for KalugaOrientation {
        fn mode(&self) -> u8 {
            match self {
                Self::Portrait => 0,
                Self::Landscape => 0x20 | 0x40,
                Self::PortraitFlipped => 0x80 | 0x40,
                Self::LandscapeFlipped => 0x80 | 0x20,
            }
        }

        fn is_landscape(&self) -> bool {
            matches!(self, Self::Landscape | Self::LandscapeFlipped)
        }
    }

    info!("About to initialize the Kaluga ILI9341 SPI LED driver");

    let config = <spi::config::Config as Default>::default()
        .baudrate(40.MHz().into());
        //.bit_order(spi::config::BitOrder::MSBFirst);

        let mut backlight = gpio::PinDriver::output(backlight)?;
        backlight.set_low()?;
        
    
        let di = SPIInterfaceNoCS::new(
            spi::SpiDeviceDriver::new_single(
                spi,
                sclk,
                sdo,
                Option::<gpio::AnyIOPin>::None,
                spi::Dma::Disabled,
                Some(cs),
                &spi::SpiConfig::new().baudrate(40.MHz().into()),
            )?,
            gpio::PinDriver::output(dc)?,
        );
    
        let reset = gpio::PinDriver::output(rst)?;
    
        ili9341::Ili9341::new(
            di,
            reset,
            &mut delay::Ets,
            KalugaOrientation::Landscape,
            // KalugaOrientation::LandscapeFlipped // uncomment this line and comment the line above for correct Wokwi simulation
            ili9341::DisplaySize240x320,
        ).map_err(|e| anyhow!("Failed to init display"))
}

#[cfg(feature = "kaluga_st7789")]
pub(crate) fn kaluga_create_display_st7789(
    backlight: gpio::Gpio6,
    dc: gpio::Gpio13,
    rst: gpio::Gpio16,
    spi: spi::SPI3,
    sclk: gpio::Gpio15,
    sdo: gpio::Gpio9,
    cs: gpio::Gpio11,
) -> Result<
    st7789::ST7789<
        SPIInterfaceNoCS<
            spi::SpiDeviceDriver<'d, 
                spi::SpiDriver<'d>
            >, 
            gpio::PinDriver<'d,
                gpio::Gpio13, 
                gpio::Output
            >
        >,
        gpio::PinDriver<'d,
                gpio::Gpio16, 
                gpio::Output
        >,  
    >,
> {
    info!("About to initialize the Kaluga ST7789 SPI LED driver");

    let config = <spi::config::Config as Default>::default()
        .baudrate(80.MHz().into())
        .bit_order(spi::config::BitOrder::MSBFirst);

    let mut backlight = gpio::PinDriver::output(backlight)?;
    backlight.set_high()?;

    let di = SPIInterfaceNoCS::new(
        spi::SpiDeviceDriver::new_single(
            spi,
            sclk,
            sdo,
            Option::<gpio::AnyIOPin>::None,
            spi::Dma::Disabled,
            Some(cs),
            &spi::SpiConfig::new().baudrate(40.MHz().into()),
        )?,
        gpio::PinDriver::output(dc)?,
    );

    let reset = gpio::PinDriver::output(rst)?;

    let mut display = st7789::ST7789::new(di, reset, 320, 240);

    display.init(&mut delay::Ets)?;
    display
        .set_orientation(st7789::Orientation::Landscape)?;

    Ok(display)
}

#[cfg(feature = "heltec")]
pub(crate) fn heltec_create_display(
    rst: gpio::Gpio19<gpio::Unknown>,
    i2c: i2c::I2C0,
    sda: gpio::Gpio4<gpio::Unknown>,
    scl: gpio::Gpio18<gpio::Unknown>,
) -> Result<
    ssd1306::Ssd1306<
        ssd1306::prelude::I2CInterface<
            i2c::Master<i2c::I2C0, gpio::Gpio4<gpio::Unknown>, gpio::Gpio18<gpio::Unknown>>,
        >,
        ssd1306::size::DisplaySize128x64,
        ssd1306::mode::BufferedGraphicsMode<ssd1306::size::DisplaySize128x64>,
    >,
> {
    info!("About to initialize the Heltec SSD1306 I2C LED driver");

    let config = <i2c::config::MasterConfig as Default>::default().baudrate(400.kHz().into());

    let di = ssd1306::I2CDisplayInterface::new(i2c::Master::<i2c::I2C0, _, _>::new(
        i2c,
        i2c::MasterPins { sda, scl },
        config,
    )?);

    let mut delay = delay::Ets;
    let mut reset = rst.into_output()?;

    reset.set_high()?;
    delay.delay_ms(1 as u32);

    reset.set_low()?;
    delay.delay_ms(10 as u32);

    reset.set_high()?;

    let mut display = ssd1306::Ssd1306::new(
        di,
        ssd1306::size::DisplaySize128x64,
        ssd1306::rotation::DisplayRotation::Rotate0,
    )
    .into_buffered_graphics_mode();

    display.init();

    Ok(display)
}

#[cfg(any(feature = "esp32s2_usb_otg", feature = "esp32s3_usb_otg"))]
pub(crate) fn esp32s2s3_usb_otg_create_display(
    backlight: gpio::Gpio9<gpio::Unknown>,
    dc: gpio::Gpio4<gpio::Unknown>,
    rst: gpio::Gpio8<gpio::Unknown>,
    spi: spi::SPI3,
    sclk: gpio::Gpio6<gpio::Unknown>,
    sdo: gpio::Gpio7<gpio::Unknown>,
    cs: gpio::Gpio5<gpio::Unknown>,
) -> Result<
    st7789::ST7789<
        SPIInterfaceNoCS<
            spi::Master<
                spi::SPI3,
                gpio::Gpio6<gpio::Unknown>,
                gpio::Gpio7<gpio::Unknown>,
                gpio::Gpio21<gpio::Unknown>,
                gpio::Gpio5<gpio::Unknown>,
            >,
            gpio::Gpio4<gpio::Output>,
        >,
        gpio::Gpio8<gpio::Output>,
    >,
> {
    info!("About to initialize the ESP32-S2/S3-USB-OTG SPI LED driver ST7789VW");

    let config = <spi::config::Config as Default>::default()
        .baudrate(80.MHz().into());
        // .bit_order(spi::config::BitOrder::MSBFirst);

    let mut backlight = backlight.into_output()?;
    backlight.set_high()?;

    let di = SPIInterfaceNoCS::new(
        spi::Master::<spi::SPI3, _, _, _, _>::new(
            spi,
            spi::Pins {
                sclk,
                sdo,
                sdi: Option::<gpio::Gpio21<gpio::Unknown>>::None,
                cs: Some(cs),
            },
            config,
        )?,
        dc.into_output()?,
    );

    let reset = rst.into_output()?;

    let mut display = st7789::ST7789::new(di, reset, 240, 240);

    display.init(&mut delay::Ets);
    display
        .set_orientation(st7789::Orientation::Landscape);

    Ok(display)
}

#[cfg(any(
    feature = "ttgo",
    feature = "esp32_ili9341",
    feature = "esp32s2_usb_otg",
    feature = "esp32s3_usb_otg",
    feature = "esp32c3_ili9341",
    feature = "kaluga_ili9341",
    feature = "kaluga_st7789",
    feature = "esp32s2_ili9341",
    feature = "esp32s3_ili9341",
    feature = "esp32c3_rust_board_ili9341"
))]
pub(crate) fn color_conv(color: ZXColor, _brightness: ZXBrightness) -> Rgb565 {
    match color {
        ZXColor::Black => Rgb565::BLACK,
        ZXColor::Blue => Rgb565::BLUE,
        ZXColor::Red => Rgb565::RED,
        ZXColor::Purple => Rgb565::MAGENTA,
        ZXColor::Green => Rgb565::GREEN,
        ZXColor::Cyan => Rgb565::CYAN,
        ZXColor::Yellow => Rgb565::YELLOW,
        ZXColor::White => Rgb565::WHITE,
    }
}


#[cfg(feature = "heltec")]
pub(crate) fn color_conv(color: ZXColor, _brightness: ZXBrightness) -> BinaryColor {
    match color {
        ZXColor::Black | ZXColor::Blue | ZXColor::Purple => BinaryColor::Off,
        _ => BinaryColor::On,
    }
}

// 0 - > 8   1 --> 10