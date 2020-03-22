

#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate panic_halt;
extern crate stm32f4xx_hal as hal;

use cortex_m_rt::entry;


use crate::hal::{
    i2c::I2c, 
    prelude::*, 
    stm32,
    delay::Delay,
    };

use ssd1306::{
    prelude::*, 
    Builder as SSD1306Builder
    };

use embedded_graphics::{
    //geometry::Point,
    //pixelcolor::BinaryColor,
    prelude::*,
    //image::{Image, ImageRaw},
    image::Image1BPP,
    };

use tinybmp::Bmp;


const BOOT_DELAY_MS: u16 = 100; 

#[entry]
fn main() -> ! {
    if let (Some(dp), Some(cp)) = (
        stm32::Peripherals::take(),
        cortex_m::peripheral::Peripherals::take(),
) {
        // Set up the system clock. We want to run at max speed. 
        // High speed external clock from the external 8 MHz crystal
        // PCLK1 (internal APB1 clock frequency) set to the maximum
        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.use_hse(8.mhz()).sysclk(168.mhz()).pclk1(42.mhz()).freeze();
        
        let mut delay = Delay::new(cp.SYST, clocks);
        
        //delay necessary for the I2C to initiate correctly and start on boot without having to reset the board

        delay.delay_ms(BOOT_DELAY_MS);

        // Set up I2C - SCL is PB8 and SDA is PB9; they are set to Alternate Function 4
        
        let gpiob = dp.GPIOB.split();
        let scl = gpiob.pb8.into_alternate_af4().set_open_drain();
        let sda = gpiob.pb9.into_alternate_af4().set_open_drain();
        let i2c = I2c::i2c1(dp.I2C1, (scl, sda), 400.khz(), clocks);
        
        // Set up the display
        
        let mut disp: GraphicsMode<_> = SSD1306Builder::new().connect_i2c(i2c).into();
        
        disp.init().unwrap();

        

        //let bmp = Bmp::from_slice(include_bytes!("./dvd.bmp")).expect("Failed to load BMP image");

        //let im: Image<Bmp, Rgb565> = Image::new(&bmp, Point::new(0, 0));

        //let mut arr: [Image<Bmp, Rgb565>; 2] = [Image::new(&bmp, Point::zero());2];

         // The display uses `BinaryColor` pixels (on/off only). Here, we `map()` over every pixel
        // and naively convert the color to an on/off value. The logic below simply converts any
        // color that's not black into an "on" pixel.
        
        /*
        arr[0].into_iter()
        .map(|Pixel(position, color)| {
            Pixel(
                position,
                if color != Rgb565::BLACK {
                    BinaryColor::On
                } else {
                    BinaryColor::Off
                },
            )
        })
        .draw(&mut disp)
        .unwrap();
        */

        

        //let image: ImageRaw<BinaryColor> = ImageRaw::new(&[0xff,0x00,0xff,0x00], 4, 4);

        //let mut image: Image<_, BinaryColor> = Image::new(&image, Point::zero());
        
        //image.translate_mut(Point::new(10,20));

        //im.draw(&mut disp).unwrap();

        let im = Image1BPP::new(include_bytes!("../resources/dvd.data"), 55, 24);
        
        //im.into_iter().draw(&mut disp).unwrap();
        
        disp.draw(im.into_iter());
        disp.flush().unwrap();
        
        loop {}


    }
      
        loop {}

    

}
