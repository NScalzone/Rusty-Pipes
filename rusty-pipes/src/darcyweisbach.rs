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
    // Update the flow at a node
    fn update_flow(&mut self, to_add: f64) {
        self.flow += to_add;
    }

    // Update the pressure at a node
    fn update_pressure(&mut self, to_add: f64) {
        self.pressure += to_add;
    }

    // Add a connection to a node
    pub fn add_connection(&mut self, pipe_to_add: f64) {
        self.connections += 1;
        self.connecting_pipes.push(pipe_to_add);
    }
}

// Construtor function for node struct
pub fn node_constructor(
    node_number: f64,
    pressure: f64,
    flow: f64,
    connections: usize,
    connecting_pipes: Vec<f64>,
) -> Node {
    Node {
        node_number,
        pressure,
        flow,
        connections,
        connecting_pipes,
    }
}

// Struct to hold all of the data for a pipe in the system
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
    // Function to update the flow in a pipe
    fn update_flow(&mut self, to_add: f64) {
        self.flow += to_add;
    }

    // Function to update the velocity in a pipe
    fn update_velocity(&mut self, to_add: f64) {
        self.velocity += to_add;
    }
}

// Pipe struct constructor
pub fn pipe_constructor(
    pipe_number: f64,
    length: f64,
    diameter: f64,
    velocity: f64,
    flow: f64,
    start_node: f64,
    end_node: f64,
) -> Pipe {
    Pipe {
        pipe_number,
        length,
        diameter,
        velocity,
        flow,
        start_node,
        end_node,
    }
}

// Function to display node data
pub fn display_node(node: &Node) {
    println!(
        "Node Number: {}
    Pressure: {} psi
    Flow Rate: {} GPM
    # of Pipe Connections: {}
    Connects to pipes:",
        node.node_number, node.pressure, node.flow, node.connections
    );
    for i in &node.connecting_pipes {
        println!("\t{}", i);
    }
}

// Function to display pipe data
pub fn display_pipe(pipe: &Pipe) {
    println!(
        "pipe_number: {}
        length: {} ft
        diameter: {} inches
        velocity: {} ft/s
        flow: {} GPM
        start_node: {}
        end_node: {}",
        pipe.pipe_number,
        pipe.length,
        pipe.diameter,
        pipe.velocity,
        pipe.flow,
        pipe.start_node,
        pipe.end_node
    );
}

// This function takes a flow-rate in Gallons per minute and converts to cubic feet per second
fn convert_flowrate(flow: f64) -> f64 {
    (flow * 0.1337) / 60.0
}
#[test]
fn test_convert_flowrate() {
    let testflow = 10.0; // test flow of 10 GPM
    let test_cfs_flow = convert_flowrate(testflow);
    assert_eq!(0.022283333333333336, test_cfs_flow);
}

// This takes in a diameter in inches, and returns an area in square feet
fn get_area(diameter: f64) -> f64 {
    let pi = f64::consts::PI;
    0.00694 * pi * ((diameter / 2.0) * (diameter / 2.0))
}

#[test]
fn test_get_area() {
    let test_diameter = 2.0; // test diameter of 2"
    let test_area = get_area(test_diameter);
    assert_eq!(0.021802653015913165, test_area);
}

// This takes in a flow rate in Gallons per Minute, and an diameter in inches, and returns velocity in feet per second
fn get_velocity(flow: f64, diameter: f64) -> f64 {
    // get area in square feet
    let area = get_area(diameter);
    // get flow in cubic feet per second
    let cfs_flow = convert_flowrate(flow);
    cfs_flow / area
}

#[test]
fn test_get_velocity() {
    let test_diameter = 2.0; // test diameter 2"
    let test_flow = 10.0; // test flow 10 GPM
    let test_velocity = get_velocity(test_flow, test_diameter);
    assert_eq!(1.0220468727851302, test_velocity);
}

// Application of the Darcy-Weisbach equation to find pressure loss across the pipe.
fn pressure_loss(length: f64, velocity: f64, diameter: f64) -> f64 {
    let rho = 0.0361; // density of water in lb/in^3
    let mew = 0.000020337; //dynamic viscosity lbf*s/ft^2, from https://www.engineeringtoolbox.com/water-dynamic-kinematic-viscosity-d_596.html
    let reynolds_number = (rho * velocity * (diameter)) / (mew / 144.0);
    let roughness = 0.00025;
    let numerator = ((roughness / diameter) / 3.7) + (5.74 / reynolds_number.powf(0.9));
    let friction = 0.25 / ((numerator.log10()) * (numerator.log10()));
    length * (friction * (rho / 2.0) * ((velocity * velocity) / diameter))
}
#[test]
fn test_pressure_loss() {
    let test_diameter = 2.0; // test diameter 2"
    let test_flow = 10.0; // test flow 10 GPM
    let test_length = 100.0; // test pipe length 100'
    let test_velocity = get_velocity(test_flow, test_diameter);
    let test_pressure_loss = pressure_loss(test_length, test_velocity, test_diameter);
    assert_eq!(0.013842447133685504, test_pressure_loss);
}

// Function to find the index in the list of pipes for a specific pipe number
fn find_pipe_index(pipes: &[Pipe], pipe_number: f64) -> usize {
    let mut index: usize = 0;
    for (i, pipe) in pipes.iter().enumerate() {
        if pipe.pipe_number == pipe_number {
            index = i;
        }
    }
    index
}

// Function to find the index in the list of nodes for a specific node number
fn find_node_index(nodes: &[Node], node_number: f64) -> usize {
    let mut index: usize = 0;
    for (i, node) in nodes.iter().enumerate() {
        if node.node_number == node_number {
            index = i;
        }
    }
    index
}

// function to return the first pipe's flow in a 2-way junction
fn divide_flow_2_ways(diameter1: f64, diameter2: f64, flow: f64) -> f64 {
    let area1 = get_area(diameter1);
    let area2 = get_area(diameter2);
    let total_area = area1 + area2;
    let pipe_1_percentage = area1 / total_area;
    flow * pipe_1_percentage
}

// Function to return the first pipe's flow in a 3-way junction
fn divide_flow_3_ways(diameter1: f64, diameter2: f64, diameter3: f64, flow: f64) -> f64 {
    let area1 = get_area(diameter1);
    let area2 = get_area(diameter2);
    let area3 = get_area(diameter3);
    let total_area = area1 + area2 + area3;
    let pipe_1_percentage = area1 / total_area;
    flow * pipe_1_percentage
}

// System calculation main function
pub fn calculate_system(pipes: &mut [Pipe], nodes: &mut [Node]) {
    for i in 0..nodes.len() {
        // node 1 will have pressure/flow values
        if nodes[i].connections == 1 {
            let pipe_out = nodes[i].connecting_pipes[0];
            let pipe_index = find_pipe_index(pipes, pipe_out);
            if pipes[pipe_index].flow == 0.0 {
                pipes[pipe_index].update_flow(nodes[i].flow);
            }
            let velocity = get_velocity(pipes[pipe_index].flow, pipes[pipe_index].diameter);
            pipes[pipe_index].update_velocity(velocity);
            let pressure_loss = pressure_loss(
                pipes[pipe_index].length,
                pipes[pipe_index].velocity,
                pipes[pipe_index].diameter,
            );
            let next_node_pressure = nodes[i].pressure - pressure_loss;
            let next_node = find_node_index(nodes, pipes[pipe_index].end_node);
            if nodes[i].node_number != nodes[next_node].node_number {
                nodes[next_node].update_pressure(next_node_pressure);
                nodes[next_node].update_flow(pipes[pipe_index].flow);
            }
        } else if nodes[i].connections == 2 {
            let first_pipe = find_pipe_index(pipes, nodes[i].connecting_pipes[0]);
            let second_pipe = find_pipe_index(pipes, nodes[i].connecting_pipes[1]);
            let first_pipe_flow = divide_flow_2_ways(
                pipes[first_pipe].diameter,
                pipes[second_pipe].diameter,
                nodes[i].flow,
            );
            let second_pipe_flow = nodes[i].flow - first_pipe_flow;
            pipes[first_pipe].update_flow(first_pipe_flow);
            let first_veloctiy = get_velocity(pipes[first_pipe].flow, pipes[first_pipe].diameter);
            pipes[first_pipe].update_velocity(first_veloctiy);
            pipes[second_pipe].update_flow(second_pipe_flow);
            let second_velocity =
                get_velocity(pipes[second_pipe].flow, pipes[second_pipe].diameter);
            pipes[second_pipe].update_velocity(second_velocity);

            let first_pressure_loss = pressure_loss(
                pipes[first_pipe].length,
                pipes[first_pipe].velocity,
                pipes[first_pipe].diameter,
            );
            let first_next_node_pressure = nodes[i].pressure - first_pressure_loss;
            let first_next_node = find_node_index(nodes, pipes[first_pipe].end_node);
            nodes[first_next_node].update_pressure(first_next_node_pressure);
            nodes[first_next_node].update_flow(pipes[first_pipe].flow);

            let second_pressure_loss = pressure_loss(
                pipes[second_pipe].length,
                pipes[second_pipe].velocity,
                pipes[second_pipe].diameter,
            );
            let second_next_node_pressure = nodes[i].pressure - second_pressure_loss;
            let second_next_node = find_node_index(nodes, pipes[second_pipe].end_node);
            nodes[second_next_node].update_pressure(second_next_node_pressure);
            nodes[second_next_node].update_flow(pipes[second_pipe].flow);
        } else if nodes[i].connections == 3 {
            let first_pipe = find_pipe_index(pipes, nodes[i].connecting_pipes[0]);
            let second_pipe = find_pipe_index(pipes, nodes[i].connecting_pipes[1]);
            let third_pipe = find_pipe_index(pipes, nodes[i].connecting_pipes[2]);
            let first_pipe_flow = divide_flow_3_ways(
                pipes[first_pipe].diameter,
                pipes[second_pipe].diameter,
                pipes[third_pipe].diameter,
                nodes[i].flow,
            );
            let remaining_flow = nodes[i].flow - first_pipe_flow;
            let second_pipe_flow = divide_flow_2_ways(
                pipes[second_pipe].diameter,
                pipes[third_pipe].diameter,
                remaining_flow,
            );
            let third_pipe_flow = remaining_flow - second_pipe_flow;
            pipes[first_pipe].update_flow(first_pipe_flow);
            let first_veloctiy = get_velocity(pipes[first_pipe].flow, pipes[first_pipe].diameter);
            pipes[first_pipe].update_velocity(first_veloctiy);
            pipes[second_pipe].update_flow(second_pipe_flow);
            let second_velocity =
                get_velocity(pipes[second_pipe].flow, pipes[second_pipe].diameter);
            pipes[second_pipe].update_velocity(second_velocity);
            pipes[third_pipe].update_flow(third_pipe_flow);
            let third_veloctiy = get_velocity(pipes[third_pipe].flow, pipes[third_pipe].diameter);
            pipes[third_pipe].update_velocity(third_veloctiy);

            let first_pressure_loss = pressure_loss(
                pipes[first_pipe].length,
                pipes[first_pipe].velocity,
                pipes[first_pipe].diameter,
            );
            let first_next_node_pressure = nodes[i].pressure - first_pressure_loss;
            let first_next_node = find_node_index(nodes, pipes[first_pipe].end_node);
            nodes[first_next_node].update_pressure(first_next_node_pressure);
            nodes[first_next_node].update_flow(pipes[first_pipe].flow);

            let second_pressure_loss = pressure_loss(
                pipes[second_pipe].length,
                pipes[second_pipe].velocity,
                pipes[second_pipe].diameter,
            );
            let second_next_node_pressure = nodes[i].pressure - second_pressure_loss;
            let second_next_node = find_node_index(nodes, pipes[second_pipe].end_node);
            nodes[second_next_node].update_pressure(second_next_node_pressure);
            nodes[second_next_node].update_flow(pipes[second_pipe].flow);

            let third_pressure_loss = pressure_loss(
                pipes[third_pipe].length,
                pipes[third_pipe].velocity,
                pipes[third_pipe].diameter,
            );
            let third_next_node_pressure = nodes[i].pressure - third_pressure_loss;
            let third_next_node = find_node_index(nodes, pipes[third_pipe].end_node);
            nodes[third_next_node].update_pressure(third_next_node_pressure);
            nodes[third_next_node].update_flow(pipes[third_pipe].flow);
        } else {
            eprintln!("Error: More than 3 pipes are leaving node {}, please revise the input file to valid pipe system.", nodes[i].node_number);
            std::process::exit(1);
        }
    }
}
