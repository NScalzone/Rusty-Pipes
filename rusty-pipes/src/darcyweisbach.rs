use ndarray::Array2;
use std::f64;

// a struct representing a node, which is a junction between pipes
#[derive(Clone)]
pub struct Node {
    pub node_number: f64,
    pub pressure: f64,
    pub flow: f64,
    pub connections: usize,
    pub connecting_pipes: Vec<f64>,
}
impl Node {
    pub fn update_flow(&mut self, to_add: f64) {
        self.flow += to_add;
    }

    pub fn update_pressure(&mut self, to_add: f64) {
        self.pressure += to_add;
    }

    pub fn add_connection(&mut self, pipe_to_add: f64) {
        self.connections += 1;
        self.connecting_pipes.push(pipe_to_add);
    }
}

pub fn node_constructor(
    node_number: f64,
    pressure: f64,
    flow: f64,
    connections: usize,
    connecting_pipes: Vec<f64>,
) -> Node {
    let temp = Node {
        node_number,
        pressure,
        flow,
        connections,
        connecting_pipes,
    };
    temp
}

#[derive(Clone)]
pub struct Pipe {
    pub pipe_number: f64,
    pub length: f64,
    pub diameter: f64,
    pub velocity: f64,
    pub flow: f64,
    pub start_node: f64,
    pub end_node: f64,
}
impl Pipe {
    pub fn update_flow(&mut self, to_add: f64) {
        self.flow += to_add;
    }
}

pub fn pipe_constructor(
    pipe_number: f64,
    length: f64,
    diameter: f64,
    velocity: f64,
    flow: f64,
    start_node: f64,
    end_node: f64,
) -> Pipe {
    let temp = Pipe {
        pipe_number,
        length,
        diameter,
        velocity,
        flow,
        start_node,
        end_node,
    };
    temp
}

pub fn display_node(node: &Node) {
    println!("Node Number: {}
    Pressure: {} psi
    Flow Rate: {} GPM
    # of Pipe Connections: {}
    Connects to pipes:",
    node.node_number, 
    node.pressure, 
    node.flow, 
    node.connections);
    for i in &node.connecting_pipes {
        println!("\t{}", i);
    }
}

pub fn display_pipe(pipe: &Pipe) {
    println!("pipe_number: {}
        length: {}
        diameter: {}
        velocity: {}
        flow: {}
        start_node: {}
        end_node: {}", 
        pipe.pipe_number, 
        pipe.length, 
        pipe.diameter, 
        pipe.velocity, 
        pipe.flow, 
        pipe.start_node, 
        pipe.end_node);
}

// pub assign_nodes (pipedata:Array2<f64>, mut start_pressure:f64, mut start_flow:f64) -> Vec<Node>{
//     let mut system = Vec::new();
//     let mut in_system = Vec::new();
//     let columns = 5;
//     let rows:usize = total_vals / columns;
//     for i in 0..rows{
//         if in_system.contains(i){

//         }
//         else{
//             if pipedata[(i,0)] != 1.0{
//                 start_pressure = 0;
//                 start_flow = 0;
//             }
//             // for new node, connections is 1 (since we'll be adding as we go)
//             let mut connecting_pipes = vec![pipedata[(i,(columns-1))]];
//             let new_node = node_constructor(pipedata[(i,0), start_pressure, start_flow, 1, connecting_pipes]);
//         }
//     }
// }

// This function takes a flow-rate in Gallons per minute and converts to cubic feet per second
pub fn convert_flowrate(flow: f64) -> f64 {
    let cubic_feet_per_second = (flow * 0.1337) / 60.0;
    cubic_feet_per_second
}

// This takes in a diameter in inches, and returns an area in square feet
pub fn get_area(diameter: f64) -> f64 {
    let pi = f64::consts::PI;
    let area = 0.00694 * pi * ((diameter / 2.0) * (diameter / 2.0));
    area
}

// This takes in a flow rate in cubic feet per second, and an area in square feet, and returns velocity in feet per second
pub fn get_velocity(flow: f64, area: f64) -> f64 {
    let velocity = flow / area;
    velocity
}

// This takes length in feet, velocity in ft/s, diameter in inches, and friction (unitless coefficient), and returns head loss in feet of head
pub fn head_loss(length: f64, velocity: f64, diameter: f64, friction: f64) -> f64 {
    let g = 32.174; // gravity constant in ft/s^2
    let d = diameter / 12.0; // diameter in feet for unit consistency
    let head_loss = friction * ((length * (velocity * velocity)) / (2.0 * g * d));
    head_loss
}

// Application of the Darcy-Weisbach equation to find pressure loss across the pipe.
pub fn pressure_loss(length: f64, velocity: f64, diameter: f64) -> f64 {
    // let g = 32.174; // gravity constant in ft/s^2
    let rho = 0.0361; // density of water in lb/in^3
    let mew = 0.000020337; //dynamic viscosity lbf*s/ft^2, from https://www.engineeringtoolbox.com/water-dynamic-kinematic-viscosity-d_596.html
    let reynolds_number = (rho * velocity * (diameter)) / (mew / 144.0);
    let roughness = 0.00025;
    println!("reynolds number is {}", reynolds_number);
    let numerator = ((roughness / diameter) / 3.7) + (5.74 / reynolds_number.powf(0.9));
    let friction = 0.25 / ((numerator.log10()) * (numerator.log10()));
    println!("friciton is {}", friction);
    let pressure_loss = length * (friction * (rho / 2.0) * ((velocity * velocity) / diameter));
    pressure_loss
}
