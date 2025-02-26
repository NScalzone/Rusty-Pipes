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

pub fn pressure_loss(head_loss:f64) -> f64{
    let g = 32.174; // gravity constant in ft/s^2
    let rho = 62.4; // density of water in lb/ft^3
    let pressure_loss = (g * rho * head_loss) / 1728.0;
    pressure_loss
}