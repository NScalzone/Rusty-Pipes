mod darcyweisbach;
use std::{error::Error, io, process};
use csv::{ReaderBuilder, WriterBuilder};
use ndarray::{array, Array2};
use ndarray_csv::{Array2Reader, Array2Writer};
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>>{

    // Read an array back from the file
    let file = File::open("/Users/nicholasscalzone/Documents/COMPUTER SCIENCE CLASSES/Rust Programming/Rusty-Pipes/rusty-pipes/pipedata.csv")?;
    let mut reader = ReaderBuilder::new().has_headers(false).from_reader(file);
    let array_read: Array2<f64> = reader.deserialize_array2((5, 4))?;

    println!("{}", array_read);
    Ok(())
    // Ensure that we got the original array back
    
    // let flow = 38.0975;
    // println!("Starting with {} GPM", flow);
    // let pipe_diameter = 2.0;
    // let pipe_length = 500.0;

    // let foot_flow = darcyweisbach::convert_flowrate(flow);
    // let area = darcyweisbach::get_area(pipe_diameter);
    // let velocity = darcyweisbach::get_velocity(foot_flow, area);
    // let friction = 0.015; 
    // let head_loss = darcyweisbach::head_loss(pipe_length, velocity, pipe_diameter, friction);
    // let pressure_loss = darcyweisbach::pressure_loss(pipe_length, velocity, pipe_diameter);
    // println!("In cubic feet per second, that is: {}",foot_flow);
    // println!("For an diameter of {} inches, area is {}", pipe_diameter, area);
    // println!("Velocity in the pipe is {}ft/s", velocity);
    // println!("Head loss in the pipe is {}ft", head_loss);
    // println!("Pressure loss is {}psi", pressure_loss);
}
