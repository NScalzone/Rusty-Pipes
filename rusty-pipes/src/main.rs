mod darcyweisbach;
use csv::ReaderBuilder;
use ndarray::Array2;
use ndarray_csv::Array2Reader;
use std::error::Error;
use std::fs::File;
use std::env;
use std::str::FromStr;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3{
        eprintln!("Error: Please pass in two arguments, the starting pressure and the starting flow as float values, in that order.");
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
        let mut newpipe = darcyweisbach::pipe_constructor(pipe_number, pipe_length, diameter, 0.0, 0.0, start_node, end_node);
        pipes.push(newpipe);
    }

    for i in 0..pipes.len(){
        // darcyweisbach::display_pipe(&pipes[i]);
        let mut add_node = true;
        for k in &nodes {
            if k.node_number == pipes[i].start_node {
                add_node = false;
            }
        }
        if add_node {

        }
    }




    // let mut system = Vec::with_capacity(rows);
    // system.push(1);
    // system.push(2);
    // for i in system {
    //     println!("{}", i);
    // }
    // let mut testnode = darcyweisbach::node_constructor(1.0, 0.0, 0.0, 2, vec![1.0,2.0]);
    // let mut testnode2 = darcyweisbach::node_constructor(2.0,0.0, 0.0, 2, vec![4.0,5.0]);
    // darcyweisbach::display_node(&testnode);
    // testnode.update_flow(2.0);
    // testnode.update_pressure(35.0);
    // testnode.add_connection(3.0);
    // darcyweisbach::display_node(&testnode);

    // println!("Creating vector of nodes");

    // let mut system: Vec<darcyweisbach::Node> = Vec::new();
    // let mut nodenum = 1.0;
    // for i in 0..4 {
    //     let mut testnode = darcyweisbach::node_constructor(nodenum, 0.0, 0.0, 2, vec![1.0, 2.0]);
    //     system.push(testnode);
    //     nodenum += 1.0;
    // }

    // system.push(testnode2);

    // for j in 0..4 {
    //     darcyweisbach::display_node(&system[j]);
    // }

    // for k in system {
    //     if k.node_number == 1.0 {
    //         println!("found node 1")
    //     }
    // }

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
