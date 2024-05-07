use rppal::gpio::{Gpio, OutputPin};
use rppal::i2c::I2c;
use syact::act::StateActuator;
use syact::meas::take_simple_meas;
use syact::prelude::*;
use sybot::prelude::*;

use crate::config::{DrakeConfig, DrakeHardware};
use crate::servo_table::ServoTable;
use crate::user_terminal::UserTerminal;

// Submodules
    pub mod config;

    pub mod drawing;

    pub mod routines;

    pub mod servo_table;

    pub mod user_terminal;
// 

// Robots
    #[derive(StepperActuatorGroup)]
    pub struct DrakeComponents {
        pub x : LinearAxis<Stepper<OutputPin, OutputPin>>,
        pub y : LinearAxis<Stepper<OutputPin, OutputPin>>,
        pub z : LinearAxis<Stepper<OutputPin, OutputPin>>
    }

    pub type DrakeRobot = StepperRobot<DrakeComponents, &dyn StepperActuator, 2>;

    pub fn drake_robot_new(hw : &DrakeHardware, config : &DrakeConfig, gpio : &Gpio) -> DrakeRobot {
        DrakeRobot::new([
            AngleConfig {
                offset: config.offset_x,
                counter: false
            },
            AngleConfig {
                offset: config.offset_y,
                counter: false
            },
            AngleConfig {
                offset: config.offset_z,
                counter: false
            }
        ], DrakeComponents {
            x: LinearAxis::new(
                Stepper::new(GenericPWM::new(gpio.get(hw.x_step).unwrap().into_output(), gpio.get(hw.x_dir).unwrap().into_output()).unwrap(), StepperConst::MOT_17HE15_1504S)
                    .add_interruptor_inline(Box::new(
                        EndSwitch::new(false, Some(Direction::CW), gpio.get(hw.x_meas_pos)?.into_input())
                            .setup_inline().unwrap()
                    ))
                    .add_interruptor_inline(Box::new(
                        EndSwitch::new(false, Some(Direction::CCW), gpio.get(hw.x_meas_neg)?.into_input())
                            .setup_inline().unwrap()
                    ))
                , config.ratio_x
            ),
            y: LinearAxis::new(
                Stepper::new(GenericPWM::new(gpio.get(hw.y_step).unwrap().into_output(), gpio.get(hw.y_dir).unwrap().into_output()).unwrap(), StepperConst::MOT_17HE15_1504S)
                    .add_interruptor_inline(Box::new(
                        EndSwitch::new(false, Some(Direction::CW), gpio.get(hw.y_meas_pos).unwrap().into_input())
                            .setup_inline().unwrap()
                    ))
                , config.ratio_y
            ),
            z: LinearAxis::new(
                Stepper::new(GenericPWM::new(gpio.get(hw.z_step).unwrap().into_output(), gpio.get(hw.z_dir).unwrap().into_output()).unwrap(), StepperConst::MOT_17HE15_1504S)
                    .add_interruptor_inline(Box::new(
                        EndSwitch::new(false, Some(Direction::CW), gpio.get(hw.z_meas_neg).unwrap().into_input())
                            .setup_inline().unwrap()
                    ))
                , config.ratio_z
            )
        }, Vec::new())
    }
// 

// Station
    pub struct DrakeStation { 
        pub servo_table : ServoTable,
        pub user_terminal : UserTerminal,

        pub home : [Phi; 3]
    }

    impl DrakeStation {
        pub fn new(i2c : I2c, hw : &DrakeHardware, config : &DrakeConfig, gpio : &Gpio) -> Self {
            Self {
                servo_table: ServoTable::new(i2c).unwrap(), // TODO: Find solution without unwrap
                user_terminal: UserTerminal::new(
                    gpio,
                    hw.ut_start_switch,
                    hw.ut_start_led,
                    hw.ut_stop_switch,
                    hw.ut_stop_led
                ),
                home: config.home
            }
        }
    }

    impl Setup for DrakeStation {
        fn setup(&mut self) -> Result<(), syact::Error> {
            self.servo_table.setup()?;
            self.user_terminal.setup()?;
            
            Ok(())
        }
    }

    impl Station<DrakeComponents, dyn StepperActuator, 2> for DrakeStation {
        type Robot = LinearXYStepperRobot;

        fn home(&mut self, rob : &mut Self::Robot) -> Result<(), sybot::Error> {
            dbg!(take_simple_meas(&mut rob.comps_mut().x, &MEAS_DATA_X, SpeedFactor::MAX)?);
            dbg!(take_simple_meas(&mut rob.comps_mut().y, &MEAS_DATA_Y, SpeedFactor::MAX)?);
            dbg!(take_simple_meas(&mut rob.comps_mut().z, &MEAS_DATA_Z, SpeedFactor::MAX)?);

            dbg!(rob.move_abs_j_sync(self.home, SpeedFactor::from(0.75))?);   

            Ok(())
        }
    }
// 