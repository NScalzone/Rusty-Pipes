# Rusty-Pipes
This is a Rust command line tool to calculate the pressure loss through a piping system.

To run this program, ensure that the csv, ndarray, ndarray-csv, num-traits crates are included as dependencies in the Cargo.toml file. From there, compile the program with Cargo Build, or use Cargo Run <pressure> <flow>. You need to input pressure and flow as whole numbers.

The program uses pounds per square inch (psi) as the units for pressure, and gallons per minute (GPM). In the future, I'd like to add conversion functionality for user's who are working with metric units, but I went with the US standard for now based on my industry experience.

In order for the program to run, you need to include a pipedata.csv file. The file should have 5 columns, in the order shown below, and use floating point values for each item in the csv file. Here is an example:

Start Node | Pipe Length | Pipe Diameter | End Node | Pipe Number
-----------------------------------------------------------------
1.0        | 50.0        | 2.0           | 2.0      | 1.0
-----------------------------------------------------------------
2.0        | 50.0        | 1.0           | 3.0      | 2.0
-----------------------------------------------------------------
2.0        | 20.0        | 0.5           | 4.0      | 3.0
-----------------------------------------------------------------
3.0        | 100.0       | 0.75          | 5.0      | 4.0
-----------------------------------------------------------------

As you can see, this allows a pipe model to be developed node-to-node. The program will exit with error if more than 3 pipes leave a node, as it is unlikely to have a fitting that creates a junction of more than 4 total pipes (1 in, 3 out). The pressure and flow values from the command line input will be assigned to the first node, and the system will calculate node to node from there, balancing the flows based on the area ratio of the pipes leaving a node when a node with more than one pipe leaving it is encountered.

The program is calibrated for standard carbon steel pipe. A future expansion to include pipe roughness data for other pipe materials would make this a more versatile program.

The output of the program is a list of each pipe, with the name, diameter, length, fluid velocity, and connecting nodes listed.

The program then outputs a list of each node, with the node number, pressure, flow rate, and pipes connecting to that node.

All of this data is included for the following reasons:
- Pipe specs and connections are included to allow the user to double check their inputs.
- Pipe velocity is used by engineers to verify the correct pipe sizing was chosen for the system. Typically, velocity in the range of 5 to 8 ft/s is ideal. This is by no means a law of nature, but as a rule-of-thumb it's a good place to start.
- Pressure and flow rate at each node are the most important metrics in a system, as this will tell the engineer if the starting pressure/flow is adequate to acheive the desired end result at each point in the system.
- Connecting pipes from nodes are also provided as a double check.

Testing:
Extensive manual testing was performed through the development of this program. First, I used some online calculators on a basic pipe to ensure my pressure drop calculation was accurate. Unit tests for these basic calculations are included in the crate.

I then tested as I went to make sure my program builds given the correct format of the csv file, and to ensure that the calculation progresses through the system without error. I had to add some small bug fixes into my initial algorithm to ensure that end nodes were not treated as a cycle. The program will calculate any system presented in the format described.