use rppal::gpio::{Gpio, InputPin, Level, OutputPin};
use syact::Setup;

pub struct UserTerminal {
    switch_start : InputPin,
    led_start : OutputPin,

    switch_halt : InputPin,
    led_halt : OutputPin,
}

impl UserTerminal {
    pub fn new(gpio : &Gpio, switch_start_pin : u8, led_start_pin : u8, switch_halt_pin : u8, led_halt_pin : u8) -> Result<Self, syact::Error> {
        Ok(Self {
            switch_start: gpio.get(switch_start_pin)?.into_input(),
            led_start: gpio.get(led_start_pin)?.into_output_low(),
            
            switch_halt: gpio.get(switch_halt_pin)?.into_input(),
            led_halt: gpio.get(led_halt_pin)?.into_output_low()
        })
    }

    // Buttons
        pub fn check_start(&self) -> bool {
            self.switch_start.is_high()
        }

        pub fn check_halt(&self) -> bool {
            // Halt button signal is inversed for safety reasons
            self.switch_halt.is_low()
        }
    // 

    // LEDS
        pub fn is_start_led_on(&self) -> bool {
            self.led_start.is_set_high()
        }

        pub fn set_start_led(&mut self, value : bool) {
            self.led_start.write(Level::from(value))
        }

        pub fn is_halt_led_on(&self) -> bool {
            self.led_halt.is_set_high()
        }

        pub fn set_halt_led(&mut self, value : bool) {
            self.led_start.write(Level::from(value))
        }
    // 
}

impl Setup for UserTerminal {
    fn setup(&mut self) -> Result<(), syact::Error> {
        // self.switch_start.setup()?;
        // self.led_start.setup()?;

        // self.switch_halt.setup()?;
        // self.led_halt.setup()?;

        Ok(())
    }
}