use std::io::Write;

use clap::{command, arg, value_parser};

// use drake::drawing::{convert_line, load_points};
// use indicatif::ProgressBar;
use log::info;
use syact::prelude::*;
use sybot::prelude::*;

use drake::{drake_robot_new, DrakeStation};
use drake::config::{DrakeConfig, DrakeEnvironment, DrakeHardware};

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
        info!("#############");
        info!("# DRAI-CTRL #");
        info!("#############");

        info!("> Loading controlls ... ");
    // 

    // Config
        let hardware = DrakeHardware::parse_from_env().unwrap();
        info!("| > Loading hardware from variables done!");

        let environment = DrakeEnvironment::parse_from_env().unwrap();
        info!("| > Loading environment from variables done!");

        let config = DrakeConfig::parse_from_file(&environment.config_path).unwrap();
        info!("| > Loading config at path '{}' ... ", &environment.config_path); 
    // 

    // Hardware
        let gpio = rppal::gpio::Gpio::new().unwrap();
        info!("| > Loading GPIO done!");

        let i2c = rppal::i2c::I2c::new().unwrap();
        info!("| > Loading I2C done!");
    // 

    // RDS
        let mut rob = drake_robot_new(&hardware, &config, &gpio).unwrap();
        let mut stat = DrakeStation::new(&hardware, &config, &gpio, i2c).unwrap();
    // 

    // Init
    rob.comps_mut().set_config(StepperConfig::new(hardware.voltage, None));
    rob.comps_mut().apply_inertias(&config.weights);
    rob.setup().unwrap();

    stat.setup().unwrap();
    stat.servo_table.set_all_open().unwrap();

    let cmd = command_opt.unwrap_or(String::from("help"));

    info!("> Executing command: '{}'", cmd);

    if cmd == "draw_file" {
        stat.user_terminal.prompt_start();

        stat.home(&mut rob)?;
        
        rob.await_inactive()?;

        stat.servo_table.set_all_closed().unwrap();

        rob.move_abs_j(config.drawing_origin, Factor::new(0.5)).unwrap();

        rob.await_inactive()?;

        /* let path = arg1_opt.unwrap();

        let lines = load_points(&path);
        let pb = ProgressBar::new(lines.contour.len() as u64);

        // Safe to use
        let mut last_point = unsafe { core::mem::zeroed() };
        

        if let Some(&init_line) = lines.contour.first() {
            let [ p1, _ ] = convert_line(init_line);
            stat.reposition_pen(&mut rob, p1).unwrap();   
            last_point = p1;
        }

        for line in lines.contour {
            let [ p1, p2 ] = convert_line(line);

            if p1 != last_point {
                stat.reposition_pen(&mut rob, p1).unwrap();
            }

            // log::debug!("Driving to {:.unwrap()}", p2);
            rob.move_abs_j([ p2[0] + Delta(stat.drawing_origin[0].0), p2[1] + Delta(stat.drawing_origin[1].0), stat.drawing_origin[2]], Factor::new(0.5)).unwrap();
            rob.await_inactive().unwrap();
            
            last_point = p2;

            pb.inc(1);
        }

        pb.finish_with_message("done");
        */
    } else if cmd == "calibrate_x" {
        stat.user_terminal.prompt_start();

        info!("> Driving to home position ... ");
        stat.home(&mut rob)?;

        info!("> Starting from X-position: {}", rob.gammas()[0]);

        loop {
            let mut buffer = String::new();

            print!("| > New X-Pos: "); 
            std::io::stdout().flush().unwrap();
            std::io::stdin().read_line(&mut buffer).unwrap();
    
            let new_pos : f32 = buffer.trim().parse().unwrap();

            rob.comps_mut().x.drive_abs(Gamma(new_pos), Factor::HALF).unwrap();
        }

    } else if cmd == "calibrate_y" {
        stat.user_terminal.prompt_start();

        info!("> Driving to home position ... ");
        stat.home(&mut rob)?;

        info!("> Starting from Y-position: {}", rob.gammas()[1]);

        loop {
            let mut buffer = String::new();

            print!("| > New Y-Pos: "); 
            std::io::stdout().flush().unwrap();
            std::io::stdin().read_line(&mut buffer).unwrap();
    
            let new_pos : f32 = buffer.trim().parse().unwrap();

            rob.comps_mut().y.drive_abs(Gamma(new_pos), Factor::HALF).unwrap();
        }

    } else if cmd == "calibrate_z" {
        stat.user_terminal.prompt_start();

        info!("> Driving to home position ... ");
        stat.home(&mut rob)?;

        stat.servo_table.set_all_closed().unwrap();

        info!("> Starting from Z-position: {}", rob.comps().z.gamma());

        loop {
            let mut buffer = String::new();

            print!("| > New Z-Pos: "); 
            std::io::stdout().flush().unwrap();
            std::io::stdin().read_line(&mut buffer).unwrap();
    
            let new_pos : f32 = buffer.trim().parse().unwrap();

            rob.comps_mut().z.drive_abs(Gamma(new_pos), Factor::HALF).unwrap();
        }

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
            info!("> Servos are now open!")

        } else if state == "closed" {
            stat.servo_table.set_all_closed().unwrap();
            info!("> Servos are now closed!");

        } else if state == "standby" {
            info!("> Servos are now on standby!");
            stat.servo_table.set_all_standby().unwrap();

        } else if state == "single" {
            info!("> Starting single table tester ... ");

            stat.servo_table.set_all_standby().unwrap();

            for id in 0 .. 8 {
                info!("| > Servo with id {} now open", id);
                stat.servo_table.set_servo_open(id).unwrap();

                stat.user_terminal.prompt_start();

                info!("| > Servo with id {} now closed", id);
                stat.servo_table.set_servo_open(id).unwrap();

                stat.user_terminal.prompt_start();

                stat.servo_table.set_servo_standby(id).unwrap();
            }


        } else if state == "roll" {
            info!("> Rolling servos ... ");

            stat.servo_table.roll_servos(1.0).unwrap();

            info!("| > Rolling done!");

        } else {
            info!("> Invalid state ({}) given!", state);
        }
    } else {
        info!("> Unknown command");
    }

    Ok(())
}