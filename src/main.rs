#![no_std]
#![no_main]

extern crate panic_semihosting;

pub use cortex_m::{asm::bkpt, iprint, iprintln, peripheral::ITM};
pub use cortex_m_rt::entry;
pub use f3::hal::{prelude, serial::Serial, stm32f30x::usart1, time::MonoTimer};
use f3::hal::{
    prelude::*,
    stm32f30x::{self, USART1},
};
use f3::{
    led::Led,
};
use f3::hal::delay::Delay;

fn init()
    -> (f3::hal::delay::Delay,
        &'static mut usart1::RegisterBlock,
     MonoTimer, ITM, Led) {

        let dp = stm32f30x::Peripherals::take().unwrap();
        let cp = cortex_m::Peripherals::take().unwrap();
        dp.GPIOA.odr.write(|w| unsafe { w.bits(1)});

        let mut flash = dp.FLASH.constrain();
        let mut rcc = dp.RCC.constrain();
    
        // clock configuration using the default settings (all clocks run at 8 MHz)
        let clocks = rcc.cfgr.freeze(&mut flash.acr);
    
        let mut gpioa = dp.GPIOA.split(&mut rcc.ahb);
    
        let tx = gpioa.pa9.into_af7(&mut gpioa.moder, &mut gpioa.afrh);
        let rx = gpioa.pa10.into_af7(&mut gpioa.moder, &mut gpioa.afrh);
    
        Serial::usart1(dp.USART1, (tx, rx), 115_200.bps(), clocks, &mut rcc.apb2);
        // If you are having trouble sending/receiving data to/from the
        // HC-05 bluetooth module, try this configuration instead:
        // Serial::usart1(dp.USART1, (tx, rx), 9600.bps(), clocks, &mut rcc.apb2);
    
        
        let mut gpioe = dp.GPIOE.split(&mut rcc.ahb);

        let led = gpioe.pe9
        .into_push_pull_output(
            &mut gpioe.moder,
            &mut gpioe.otyper);
        let delay = Delay::new(cp.SYST, clocks);   
        unsafe {
            (
                delay,
                &mut *(USART1::ptr() as *mut _),
                MonoTimer::new(cp.DWT, clocks),
                cp.ITM,
                led.into()
            )
        }
}

enum ButtonState {
    PressedNew,
    PressedOld,
    RiseNew,
    RiseOld
}

struct UserButton {
    last_state: bool,
}
impl UserButton {
    fn new() -> Self {
        return UserButton {last_state:false};
    }
    fn handle_user_button(&mut self) -> ButtonState {
        let result: ButtonState;
        if unsafe { (*stm32f30x::GPIOA::ptr()).idr.read().bits() & 1 != 0 } {
            if self.last_state == true {
                result = ButtonState::PressedOld;
            }
            else {
                result = ButtonState::PressedNew;
            }
            self.last_state = true;
        }
        else {
            if self.last_state == false {
                result = ButtonState::RiseOld
            }
            else {
                result = ButtonState::RiseNew;
            }
            self.last_state = false;
        }
        return result;
    }
}


#[entry]
fn main() -> ! {
    // initialize user leds
    let (mut delay, usart1, mono_timer, itm, mut led) = init();

    // Send a single character
    //usart1.tdr.write(|w| w.tdr().bits(u16::from(b'X')));
    let mut led_on = true;
    let mut user_button = UserButton::new();
    led.on();
    loop {
        match user_button.handle_user_button() {
            ButtonState::RiseNew => {
                if led_on {
                    led.off();
                }
                else {
                    led.on();
                }
                led_on = !led_on;
            },
            _ => {
                // ignoring
            }
        }
        delay.delay_ms(40_u16)
    }
}