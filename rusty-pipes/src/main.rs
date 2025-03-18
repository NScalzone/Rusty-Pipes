mod darcyweisbach;
use csv::ReaderBuilder;
use ndarray::Array2;
use ndarray_csv::Array2Reader;
use std::env;
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Error: Please pass in two arguments, the starting pressure and the starting flow as whole numbers, in that order.");
        std::process::exit(1);
    }

    let pressure: f64 = args[1].parse().unwrap();
    let flow: f64 = args[2].parse().unwrap();

    println!("pressure: {} psi, flow: {} GPM", pressure, flow);

    let columns = 5;
    // Read an array back from the file
    let file = File::open("/Users/nicholasscalzone/Documents/COMPUTER SCIENCE CLASSES/Rust Programming/Rusty-Pipes/rusty-pipes/pipedata.csv")?;
    let mut reader = ReaderBuilder::new().has_headers(false).from_reader(file);
    let array_read: Array2<f64> = reader.deserialize_array2_dynamic()?;

    let mut nodes: Vec<darcyweisbach::Node> = Vec::new();
    let mut pipes: Vec<darcyweisbach::Pipe> = Vec::new();
    // println!("{}", array_read);
    // println!("{}", array_read.len());
    let total_vals = array_read.len();
    let rows: usize = total_vals / columns;
    // println!("{} rows, {} columns", rows, columns);
    // Read data into pipe vector
    for i in 0..rows {
        let start_node = array_read[(i, 0)];
        let pipe_length = array_read[(i, 1)];
        let diameter = array_read[(i, 2)];
        let end_node = array_read[(i, 3)];
        let pipe_number = array_read[(i, 4)];
        let mut pipe_flow = 0.0;
        if i == 0 {
            pipe_flow = flow;
        }
        let newpipe = darcyweisbach::pipe_constructor(
            pipe_number,
            pipe_length,
            diameter,
            0.0,
            pipe_flow,
            start_node,
            end_node,
        );
        pipes.push(newpipe);
    }

    for i in 0..pipes.len() {
        let mut add_node = true;
        for k in &mut nodes {
            if k.node_number == pipes[i].start_node {
                add_node = false;
                k.add_connection(pipes[i].pipe_number);
            }
        }
        if add_node {
            let node_number = pipes[i].start_node;
            let mut node_pressure = 0.0;
            let mut node_flow = 0.0;
            if node_number == 1.0 {
                node_pressure = pressure;
                node_flow = flow;
            }
            let connections = 1;
            let connecting_pipes = vec![pipes[i].pipe_number];
            let newnode = darcyweisbach::node_constructor(
                node_number,
                node_pressure,
                node_flow,
                connections,
                connecting_pipes,
            );
            nodes.push(newnode);
        }
    }

    for i in 0..pipes.len() {
        let mut add_node = true;
        for k in &mut nodes {
            if k.node_number == pipes[i].end_node {
                add_node = false;
            }
        }
        if add_node {
            let node_number = pipes[i].end_node;
            let node_pressure = 0.0;
            let node_flow = 0.0;
            let connections = 1;
            let connecting_pipes = vec![pipes[i].pipe_number];
            let newnode = darcyweisbach::node_constructor(
                node_number,
                node_pressure,
                node_flow,
                connections,
                connecting_pipes,
            );
            nodes.push(newnode);
        }
    }

    darcyweisbach::calculate_system(&mut pipes, &mut nodes);

    for i in 0..pipes.len() {
        darcyweisbach::display_pipe(&pipes[i]);
    }
    for i in 0..nodes.len() {
        darcyweisbach::display_node(&nodes[i]);
    }

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
