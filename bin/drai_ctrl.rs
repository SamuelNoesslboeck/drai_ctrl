use core::time::Duration;
/* use std::io::{stdout, stdin, Read, Write}; */

use syact::prelude::*;
use sybot::prelude::*;

use drake::{drake_robot_new, DrakeStation};
use drake::config::{DrakeConfig, DrakeEnvironment, DrakeHardware};


// Process
    /* 
    fn pause() {
        let mut stdout = stdout();
        stdout.write(b"Press Enter to continue...").unwrap();
        stdout.flush().unwrap();
        stdin().read(&mut [0]).unwrap();
    } */
// 

fn main() -> Result<(), syact::Error> {
    // Init logging
        env_logger::init();
    // 

    // Header
        println!("#############");
        println!("# DRAI-CTRL #");
        println!("#############");
    // 

    // Config
        print!(" -> Loading hardware from variables ... ");
        let hardware = DrakeHardware::parse_from_env().unwrap();
        println!("done!");

        print!(" -> Loading environment from variables ... ");
        let environment = DrakeEnvironment::parse_from_env().unwrap();
        println!("done!");

        print!(" -> Loading config at path '{}' ... ", &environment.config_path); 
        let config = DrakeConfig::parse_from_file(&environment.config_path).unwrap();
        println!("done!");
    // 

    // Hardware
        print!(" -> Loading GPIO ... ");
        let gpio = rppal::gpio::Gpio::new().unwrap();
        println!("done!");

        /* 
        print!(" -> Loading I2C ... ");
        let i2c = rppal::i2c::I2c::new().unwrap();
        println!("done!"); */
    // 

    // RDS
        let mut rob = drake_robot_new(&hardware, &config, &gpio).unwrap();
        let mut stat = DrakeStation::new(&hardware, &config, &gpio).unwrap();
    // 

    // // Lines
    //     let lines = load_points(path.as_str());
    // // 

    // Init
    rob.comps_mut().set_config(StepperConfig::new(hardware.voltage, None));
    rob.comps_mut().apply_inertias(&config.weights);
    rob.setup().unwrap();

    print!("Waiting for user input ... ");

    // Wait until start has been pressed
        let mut counter = 0;

        loop {
            if (counter % 20) == 0 {
                stat.user_terminal.set_start_led(
                    !stat.user_terminal.is_halt_led_on()
                ).unwrap();
            }

            if stat.user_terminal.check_start() {
                println!("pressed!");
                break;
            }

            std::thread::sleep(Duration::from_millis(25));
            counter += 1;
        }
    // 

    println!("Driving to home position ... ");
    stat.home(&mut rob).unwrap();

    // Wait until start has been pressed
        let mut counter = 0;

        loop {
            if (counter % 20) == 0 {
                stat.user_terminal.set_start_led(
                    !stat.user_terminal.is_start_led_on()
                ).unwrap();
            }

            if stat.user_terminal.check_start() {
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

    //     log::debug!("Driving to {:.unwrap()}", p2);
    //     rob.move_abs_j(p2, draw_speed).unwrap();
    //     rob.await_inactive().unwrap();
        
    //     last_point = p2;

    //     pb.inc(1);
    // }

    // pb.finish_with_message("done");

    stat.home(&mut rob).unwrap();

    Ok(())
}