mod darcyweisbach;

fn main() {
    let flow = 100.0;
    println!("Starting with {} GPM", flow);
    let pipe_diameter = 6.0;
    let pipe_length = 500.0;

    let foot_flow = darcyweisbach::convert_flowrate(flow);
    let area = darcyweisbach::get_area(pipe_diameter);
    let velocity = darcyweisbach::get_velocity(foot_flow, area);
    let friction = 0.015; 
    let head_loss = darcyweisbach::head_loss(pipe_length, velocity, pipe_diameter, friction);
    let pressure_loss = darcyweisbach::pressure_loss(head_loss);
    println!("In cubic feet per second, that is: {}",foot_flow);
    println!("For an diameter of 6 inches, area is {}", area);
    println!("Velocity in the pipe is {}ft/s", velocity);
    println!("Head loss in the pipe is {}ft", head_loss);
    println!("Pressure loss is {}psi", pressure_loss);
}
