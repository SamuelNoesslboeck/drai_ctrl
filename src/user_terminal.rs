use rppal::gpio::{Gpio, InputPin};
use syact::Setup;
use syact::device::LED;

pub struct UserTerminal {
    switch_start : InputPin,
    led_start : LED,

    switch_halt : InputPin,
    led_halt : LED,
}

impl UserTerminal {
    pub fn new(gpio : &Gpio, switch_start_pin : u8, led_start_pin : u8, switch_halt_pin : u8, led_halt_pin : u8) -> Result<Self, syact::Error> {
        Ok(Self {
            switch_start: gpio.get(switch_start_pin)?.into_input(),
            led_start: LED::new(gpio.get(led_start_pin)?.into_output()),
            
            switch_halt: gpio.get(switch_halt_pin)?.into_input(),
            led_halt: LED::new(gpio.get(led_halt_pin)?.into_output())
        })
    }

    // Buttons
        pub fn check_start(&self) -> bool {
            self.switch_start.is_high().unwrap() // TODO: Remove unwrap
        }

        pub fn check_halt(&self) -> bool {
            // Halt button signal is inversed for safety reasons
            self.switch_halt.is_low().unwrap() // TODO: Remove unwrap
        }
    // 

    // LEDS
        pub fn is_start_led_on(&self) -> bool {
            self.led_start.is_on()
        }

        pub fn set_start_led(&mut self, value : bool) {
            self.led_start.set(value);
        }

        pub fn is_halt_led_on(&self) -> bool {
            self.led_halt.is_on()
        }

        pub fn set_halt_led(&mut self, value : bool) {
            self.led_halt.set(value);
        }
    // 
}

impl Setup for UserTerminal {
    fn setup(&mut self) -> Result<(), syact::Error> {
        self.switch_start.setup()?;
        self.led_start.setup()?;

        self.switch_halt.setup()?;
        self.led_halt.setup()?;

        Ok(())
    }
}