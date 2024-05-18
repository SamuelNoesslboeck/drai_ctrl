use core::str::FromStr;

use serde::{Serialize, Deserialize};
use syact::meas::SimpleMeasData;
use syact::MicroSteps;
use syunit::*;

pub fn parse_env<F : FromStr>(key : &str) -> Result<F, syact::Error> {
    Ok(std::env::var(key).map_err(|v| {
        format!("Failed to load from env! Var '{}' not found! Original error: {}", key, v)
    })?.parse().map_err(|_| {
        format!("Failed to load from env! Var '{}' could not be parsed!", key)
    })?)
}

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
    pub ut_halt_led : u8,
    pub ut_halt_switch : u8
}

impl DrakeHardware {
    pub fn parse_from_env() -> Result<Self, syact::Error> {
        Ok(Self {
            voltage: parse_env("DRAI_CTRL_VOLTAGE")?,

            x_step: parse_env("DRAI_X_AXIS_STEP_PIN")?,
            y_step: parse_env("DRAI_Y_AXIS_STEP_PIN")?,
            z_step: parse_env("DRAI_Z_AXIS_STEP_PIN")?,

            x_dir: parse_env("DRAI_X_AXIS_DIR_PIN")?,
            y_dir: parse_env("DRAI_Y_AXIS_DIR_PIN")?,
            z_dir: parse_env("DRAI_Z_AXIS_DIR_PIN")?,

            x_meas_pos: parse_env("DRAI_X_SWITCH_POS_PIN")?,
            x_meas_neg: parse_env("DRAI_X_SWITCH_NEG_PIN")?,

            y_meas_pos: parse_env("DRAI_Y_SWITCH_POS_PIN")?,

            z_meas_neg: parse_env("DRAI_Z_SWITCH_NEG_PIN")?,

            x_microsteps: parse_env("DRAI_X_MICROSTEPS")?,
            y_microsteps: parse_env("DRAI_Y_MICROSTEPS")?,
            z_microsteps: parse_env("DRAI_Z_MICROSTEPS")?,
            
            ut_start_led: parse_env("DRAI_UT_LED_START_PIN")?,
            ut_start_switch: parse_env("DRAI_UT_SWITCH_START_PIN")?,
            ut_halt_led: parse_env("DRAI_UT_LED_HALT_PIN")?,
            ut_halt_switch: parse_env("DRAI_UT_SWITCH_HALT_PIN")?,
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
            ctrl_dir: parse_env("DRAI_CTRL_PATH")?,
            log_path: parse_env("DRAI_LOG_PATH")?,
            config_path: parse_env("DRAI_CONFIG_PATH")?
        })
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DrakeConfig {
    pub home : [Phi; 3],
    pub drawing_origin : [Phi; 3],
    pub z_lift : Delta,

    pub ratio_x : f32,
    pub ratio_y : f32,
    pub ratio_z : f32,

    pub weights : [Inertia; 3],

    pub meas_data_x : SimpleMeasData,
    pub meas_data_y : SimpleMeasData,
    pub meas_data_z : SimpleMeasData,

    pub pixel_per_mm : f32,
    pub drawing_speed_default : f32
}

impl DrakeConfig {
    pub fn parse_from_file(path : &str) -> Result<Self, syact::Error> {
        Ok(serde_json::from_str::<Self>(
            std::fs::read_to_string(path)?.as_str()
        )?)
    }
}