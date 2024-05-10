/* use std::io::{stdout, stdin, Read, Write}; */

use clap::{command, arg, value_parser};

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

    // Cmd
        let matches = command!() 
            .about("Table testing program for the drake robot")
            .arg(arg!([command] "The command to execute").value_parser(value_parser!(String)))
            .arg(arg!([arg1] "The first argument for the command").value_parser(value_parser!(String)))
            .get_matches();

        let command_opt : Option<String> = matches.get_one::<String>("command").map(|v| v.clone());
        let arg1_opt : Option<String> = matches.get_one::<String>("arg1").map(|v| v.clone());
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

        print!(" -> Loading I2C ... ");
        let i2c = rppal::i2c::I2c::new().unwrap();
        println!("done!");
    // 

    // RDS
        let mut rob = drake_robot_new(&hardware, &config, &gpio).unwrap();
        let mut stat = DrakeStation::new(&hardware, &config, &gpio, i2c).unwrap();
    // 

    // // Lines
    //     let lines = load_points(path.as_str());
    // // 

    // Init
    rob.comps_mut().set_config(StepperConfig::new(hardware.voltage, None));
    rob.comps_mut().apply_inertias(&config.weights);
    rob.setup().unwrap();

    stat.setup().unwrap();
    stat.servo_table.set_all_open().unwrap();

    let cmd = command_opt.unwrap_or(String::from("help"));

    if cmd == "main" {
        print!(" -> Waiting for user input ... ");
        stat.user_terminal.prompt_start();
        println!("pressed!");
    
        print!(" -> Driving to home position ... ");
        stat.home(&mut rob).unwrap();
        println!("done!");
    
        stat.user_terminal.prompt_start();
    
        println!("Starting to draw ... ");

        stat.home(&mut rob).unwrap();


    } else if cmd == "calibrate" {
        stat.home(&mut rob)?;


    } else if cmd == "prompt_start" {
        stat.user_terminal.prompt_start();


    } else if cmd == "prompt_halt" {
        stat.user_terminal.prompt_halt();
        

    } else if cmd == "test_table" {
        // # test_table 
        // 
        let state = arg1_opt.unwrap_or(String::from("open"));

        if state == "open" {
            stat.servo_table.set_all_open().unwrap();
            println!("Servos are now open!")

        } else if state == "closed" {
            stat.servo_table.set_all_closed().unwrap();
            println!("Servos are now closed!");

        } else if state == "standby" {
            println!("Servos are now on standby!");
            stat.servo_table.set_all_standby().unwrap();

        } else if state == "single" {
            println!("Starting single table tester ... ");

            stat.servo_table.set_all_standby().unwrap();

            for id in 0 .. 8 {
                println!("Servo with id {} now open", id);
                stat.servo_table.set_servo_open(id).unwrap();

                stat.user_terminal.prompt_start();

                println!("Servo with id {} now closed", id);
                stat.servo_table.set_servo_open(id).unwrap();

                stat.user_terminal.prompt_start();

                stat.servo_table.set_servo_standby(id).unwrap();
            }


        } else if state == "roll" {
            println!("Rolling servos ... ");

            stat.servo_table.roll_servos(1.0).unwrap();

            println!("Rolling done!");

        } else {
            println!("Invalid state ({}) given!", state);
        }

        print!("Waiting for user input ... ");
        stat.user_terminal.prompt_start();
        println!("pressed!")


    } else {
        println!("Unknown command");
    }

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

    Ok(())
}