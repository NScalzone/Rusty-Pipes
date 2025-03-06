use std::f64;

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
pub fn get_velocity(flow: f64, area: f64) -> f64{
    let velocity = flow / area;
    velocity
}

// This takes length in feet, velocity in ft/s, diameter in inches, and friction (unitless coefficient), and returns head loss in feet of head
pub fn head_loss(length:f64, velocity:f64, diameter:f64, friction:f64) -> f64 {
    let g = 32.174; // gravity constant in ft/s^2
    let d = diameter / 12.0; // diameter in feet for unit consistency
    let head_loss = friction * ((length * (velocity * velocity))/(2.0 * g * d));
    head_loss
}

// Application of the Darcy-Weisbach equation to find pressure loss across the pipe.
pub fn pressure_loss(length:f64, velocity:f64, diameter:f64) -> f64{
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