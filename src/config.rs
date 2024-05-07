use serde::{Serialize, Deserialize};
use syact::MicroSteps;
use syunit::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DrakeHardware {
    pub voltage : f32,

    pub x_step : u8,
    pub y_step : u8,
    pub z_step : u8,
    
    pub x_dir : u8,
    pub y_dir : u8,
    pub z_dir : u8,

    pub x_meas_pos : u8,
    pub x_meas_neg : u8,

    pub y_meas_pos : u8,

    pub z_meas_neg : u8,

    pub x_microsteps : MicroSteps,
    pub y_microsteps : MicroSteps,
    pub z_microsteps : MicroSteps,

    pub ut_start_led : u8,
    pub ut_start_switch : u8,
    pub ut_stop_led : u8,
    pub ut_stop_switch : u8
}

impl DrakeHardware {
    pub fn parse_from_env() -> Result<Self, syact::Error> {
        Ok(Self {
            voltage: std::env::var("DRAI_CTRL_VOLTAGE")?.parse()?,

            x_step: std::env::var("DRAI_X_AXIS_STEP_PIN")?.parse()?,
            y_step: std::env::var("DRAI_Y_AXIS_STEP_PIN")?.parse()?,
            z_step: std::env::var("DRAI_Z_AXIS_STEP_PIN")?.parse()?,

            x_dir: std::env::var("DRAI_X_AXIS_DIR_PIN")?.parse(),
            y_dir: std::env::var("DRAI_Y_AXIS_DIR_PIN")?.parse(),
            z_dir: std::env::var("DRAI_Z_AXIS_DIR_PIN")?.parse(),

            x_meas_pos: std::env::var("DRAI_X_SWITCH_POS_PIN")?.parse(),
            x_meas_neg: std::env::var("DRAI_X_SWITCH_NEG_PIN")?.parse(),

            y_meas_pos: std::env::var("DRAI_Y_SWITCH_POS_PIN")?.parse(),

            z_meas_neg: std::env::var("DRAI_Z_SWITCH_NEG_PIN")?.parse(),

            x_microsteps: std::env::var("DRAI_X_MICROSTEPS")?.parse(),
            y_microsteps: std::env::var("DRAI_Y_MICROSTEPS")?.parse(),
            z_microsteps: std::env::var("DRAI_Z_MICROSTEPS")?.parse(),
            
            ut_start_led: std::env::var("DRAI_UT_LED_START_PIN")?.parse(),
            ut_start_switch: std::env::var("DRAI_UT_SWITCH_START_PIN")?.parse(),
            ut_stop_led: std::env::var("DRAI_UT_LED_STOP_PIN")?.parse(),
            ut_stop_switch: std::env::var("DRAI_UT_SWITCH_STOP_PIN")?.parse(),
        })
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DrakeEnvironment {
    pub ctrl_dir : String,
    pub log_path : String,
    pub config_path : String
}

impl DrakeEnvironment {
    pub fn parse_from_env() -> Result<Self, syact::Error> {
        Ok(Self {
            ctrl_dir: std::env::var("DRAI_CTRL_PATH")?,
            log_path: std::env::var("DRAI_LOG_PATH")?,
            config_path: std::env::var("DRAI_CONFIG_PATH")?
        })
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DrakeConfig {
    pub home : [Phi; 3],

    pub offset_x : Delta,
    pub offset_y : Delta,
    pub offset_z : Delta,

    pub ratio_x : f32,
    pub ratio_y : f32,
    pub ratio_z : f32,

    pub weights : [Inertia; 3],

    pub pixel_per_mm : f32,
    pub drawing_speed : f32
}

impl DrakeConfig {
    pub fn parse_from_file(path : &str) -> Result<Self, syact::Error> {
        serde_json::from_str::<Self>(
            std::fs::read_to_string(path)?.as_str()
        )
    }
}