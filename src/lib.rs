use rppal::gpio::{Gpio, OutputPin};
use rppal::i2c::I2c;
use syact::meas::take_simple_meas;
use syact::prelude::*;
use sybot::prelude::*;

use crate::config::{DrakeConfig, DrakeHardware};
use crate::servo_table::ServoTable;
use crate::user_terminal::UserTerminal;

// Submodules
    pub mod config;

    pub mod data;

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

    pub type DrakeRobot = StepperRobot<DrakeComponents, dyn StepperActuator, 3>;

    pub fn drake_robot_new(hw : &DrakeHardware, config : &DrakeConfig, gpio : &Gpio) -> Result<DrakeRobot, syact::Error> {
        Ok(DrakeRobot::new([
            AngleConfig {
                offset: Delta::ZERO,
                counter: false
            },
            AngleConfig {
                offset: Delta::ZERO,
                counter: false
            },
            AngleConfig {
                offset: Delta::ZERO,
                counter: false
            }
        ], DrakeComponents {
            x: LinearAxis::new(
                Stepper::new(
                        GenericPWM::new(
                            gpio.get(hw.x_step)?.into_output(), 
                            gpio.get(hw.x_dir)?.into_output()
                        )?, 
                        StepperConst::MOT_17HE15_1504S
                    )
                    .add_interruptor_inline(Box::new(
                        EndSwitch::new(false, Some(Direction::CW), gpio.get(hw.x_meas_pos)?.into_input())
                    ))
                    .add_interruptor_inline(Box::new(
                        EndSwitch::new(false, Some(Direction::CCW), gpio.get(hw.x_meas_neg)?.into_input())
                    ))
                , config.ratio_x
            ),
            y: LinearAxis::new(
                Stepper::new(GenericPWM::new(gpio.get(hw.y_step).unwrap().into_output(), gpio.get(hw.y_dir).unwrap().into_output()).unwrap(), StepperConst::MOT_17HE15_1504S)
                    .add_interruptor_inline(Box::new(
                        EndSwitch::new(false, Some(Direction::CW), gpio.get(hw.y_meas_pos).unwrap().into_input())
                    ))
                , config.ratio_y
            ),
            z: LinearAxis::new(
                Stepper::new(GenericPWM::new(gpio.get(hw.z_step).unwrap().into_output(), gpio.get(hw.z_dir).unwrap().into_output()).unwrap(), StepperConst::MOT_17HE15_1504S)
                    .add_interruptor_inline(Box::new(
                        EndSwitch::new(false, Some(Direction::CCW), gpio.get(hw.z_meas_neg).unwrap().into_input())
                    ))
                , config.ratio_z
            )
        }, Vec::new()))
    }
// 

// Station
    pub struct DrakeStation { 
        pub servo_table : ServoTable,
        pub user_terminal : UserTerminal,

        pub home : [Phi; 3],
        pub drawing_origin : [Phi; 3],

        pub meas_data_x : SimpleMeasData,
        pub meas_data_y : SimpleMeasData,
        pub meas_data_z : SimpleMeasData,

        // Values
        pub z_lift : f32 
    }

    impl DrakeStation {
        pub fn new(hw : &DrakeHardware, config : &DrakeConfig, gpio : &Gpio, i2c : I2c) -> Result<Self, syact::Error> {
            Ok(Self {
                servo_table: ServoTable::new(i2c)?, 
                user_terminal: UserTerminal::new(
                    gpio,
                    hw.ut_start_switch,
                    hw.ut_start_led,
                    hw.ut_halt_switch,
                    hw.ut_halt_led
                )?,

                home: config.home,
                drawing_origin: config.drawing_origin,

                meas_data_x: config.meas_data_x.clone(),
                meas_data_y: config.meas_data_y.clone(),
                meas_data_z: config.meas_data_z.clone(),

                z_lift: config.z_lift
            })
        }
        
        // pub fn into

        pub fn reposition_pen(&self, rob : &mut DrakeRobot, point : [Phi; 2]) -> Result<(), syact::Error> {
            rob.comps_mut().z.drive_abs(Gamma(self.z_lift), Factor::MAX)?;
            rob.comps_mut().x.drive_abs(Gamma(point[0].0 + self.drawing_origin[0].0), Factor::MAX)?;
            rob.comps_mut().y.drive_abs(Gamma(point[1].0 + self.drawing_origin[1].0), Factor::MAX)?;
            rob.comps_mut().z.drive_abs(Gamma(-self.z_lift), Factor::MAX)
        }
    }

    impl Setup for DrakeStation {
        fn setup(&mut self) -> Result<(), syact::Error> {
            self.servo_table.setup()?;
            self.user_terminal.setup()?;
            
            Ok(())
        }
    }

    impl Station<DrakeComponents, dyn StepperActuator, 3> for DrakeStation {
        type Robot = DrakeRobot;

        fn home(&mut self, rob : &mut Self::Robot) -> Result<(), sybot::Error> {
            self.servo_table.set_all_open()?;

            log::info!("Driving to home position ... ");

            dbg!(take_simple_meas(&mut rob.comps_mut().x, &self.meas_data_x, Factor::MAX)?);
            dbg!(take_simple_meas(&mut rob.comps_mut().y, &self.meas_data_y, Factor::MAX)?);
            dbg!(take_simple_meas(&mut rob.comps_mut().z, &self.meas_data_z, Factor::MAX)?);

            dbg!(rob.comps_mut().z.drive_abs(Gamma(self.home[2].0), Factor::new(0.75)))?;   
            dbg!(rob.comps_mut().x.drive_abs(Gamma(self.home[0].0), Factor::new(0.75)))?;
            dbg!(rob.comps_mut().y.drive_abs(Gamma(self.home[1].0), Factor::new(0.75)))?;

            log::info!(" -> Driving to home done!");

            self.servo_table.roll_servos(1.0)?;

            Ok(())
        }
    }
// 