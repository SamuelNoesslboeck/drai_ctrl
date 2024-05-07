use core::time::Duration;
use std::io::{stdout, stdin, Read, Write};

use clap::{command, arg, value_parser};
use indicatif::ProgressBar;

use syact::prelude::*;
use sybot::prelude::*;

use drake::drake_robot_new;
use drake::config::{DrakeConfig, DrakeEnvironment, DrakeHardware};


// Process
    fn pause() {
        let mut stdout = stdout();
        stdout.write(b"Press Enter to continue...").unwrap();
        stdout.flush().unwrap();
        stdin().read(&mut [0]).unwrap();
    }
// 

fn main() -> Result<(), syact::Error> {
    // Init logging
        env_logger::init();
    // 

    // Hardware
        let gpio = rppal::gpio::Gpio::new()?;
        let i2c = rppal::i2c::I2c::new()?;
    // 

    // Config
        let hardware = DrakeHardware::parse_from_env()?;
        let environment = DrakeEnvironment::parse_from_env()?;
        let config = DrakeConfig::parse_from_file(&environment.config_path)?;
    // 

    // RDS
        let mut rob = drake_robot_new(&hardware, &config, &gpio)?;
        let mut stat = DrakeStation::new(i2c, &hardware, &config, &gpio);
    // 

    // // Lines
    //     let lines = load_points(path.as_str());
    // // 

    // Init
    rob.comps_mut().set_config(StepperConfig::new(hardware.voltage, None));
    rob.comps_mut().apply_inertias(&WEIGHT_AXES);
    rob.setup().unwrap();

    println!("Driving to home position ... ");

    stat.home(&mut rob).unwrap();

    // Wait until start has been pressed
        let counter = 0;

        loop {
            if (counter % 20) {
                stat.user_terminal.set_start_led(
                    !stat.user_terminal.is_halt_led_on()
                )
            }

            if (stat.user_terminal.check_start()) {
                break;
            }

            std::thread::sleep(Duration::from_millis(25));
            counter += 1;
        }
    // 

    println!("Starting to draw ... ");

    // let pb = ProgressBar::new(lines.contour.len() as u64);

    // // Safe to use
    // let mut last_point = unsafe { core::mem::zeroed() };
    

    // if let Some(&init_line) = lines.contour.first() {
    //     let [ p1, _ ] = convert_line(init_line);
    //     stat.reposition_pen(&mut rob, p1).unwrap();   
    //     last_point = p1;
    // }

    // for line in lines.contour {
    //     let [ p1, p2 ] = convert_line(line);

    //     if p1 != last_point {
    //         stat.reposition_pen(&mut rob, p1).unwrap();
    //     }

    //     log::debug!("Driving to {:?}", p2);
    //     rob.move_abs_j(p2, draw_speed).unwrap();
    //     rob.await_inactive().unwrap();
        
    //     last_point = p2;

    //     pb.inc(1);
    // }

    // pb.finish_with_message("done");

    stat.home(&mut rob).unwrap();

    Ok(())
}