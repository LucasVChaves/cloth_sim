# 2D Real-Time Cloth Simulator

A simple, interactive 2D cloth simulator built in Rust using the macroquad library. This project implements a mass-spring model to simulate cloth physics in real-time, allowing for interactive tearing, cutting, and parameter adjustments.

<img src="./media/cloth_sim.gif" alt="Cloth Simulation Demo" width="600"/>

## Features

- **Real-time 2D Physics**: Simulates cloth behavior with stable physics.
- **Interactive Controls**: Click and drag nodes to pull the cloth.
- **Cloth Tearing**: Pulling a node with enough force will break the spring connections.
- **Cloth Cutting**: Use the right mouse button to slice through the fabric.
- **Adjustable Parameters**: A simple UI with sliders to control gravity, stiffness, tear threshold, solver iterations, and cloth dimensions in real-time.
- **Visual Feedback**: Pinned particles (anchors) are drawn in red, while mobile particles are blue.

## How It Works

The simulation is based on a [Mass-Spring Model](https://graphics.stanford.edu/~mdfisher/cloth.html), a common technique in computer graphics for simulating deformable objects. The core concepts are:

- **Discretization**: The continuous cloth is represented as a grid of point masses (particles).
- **Springs**: These particles are connected by three types of springs to maintain the cloth's structure:
- **Structural Springs**: Connect adjacent particles, resisting stretching.
- **Shear Springs**: Connect diagonal particles, resisting shearing forces.
- **Bend Springs**: Connect particles two steps away, resisting bending and adding stiffness.
- **Integration**: The physics is calculated using Verlet Integration, a numerical method that is more stable and computationally efficient for this type of constraint-based system than simpler methods like Euler integration.
- **Constraint Relaxation**: At each step, an iterative solver runs multiple times to satisfy the spring constraints (i.e., try to keep them at their resting length), resulting in a stable simulation.

## Controls

- **Left Mouse Button**: Click and drag a node to pull the cloth.
- **Right Mouse Button**: Click and drag across the cloth to cut the springs.

## Build and Run

### Prerequisites

You need to have the Rust toolchain installed on your system. If you don't have it, you can install it via [rustup.rs](https://www.rust-lang.org/learn/get-started?ref=hack-slash).

### Steps

Clone the repository:

```bash

git clone https://github.com/your-username/your-repo-name.git
cd your-repo-name
```

Build and run the project:

```Bash

cargo run --release
# Note: It is highly recommended to use the --release flag. This will enable optimizations and make the simulation run much more smoothly.
```

## Tech Stack

- **Language**: Rust
- **Graphics & Input**: macroquad - A simple and easy-to-use game library for Rust, used here for rendering, windowing, input handling, and the UI widgets.

## TODO

[ ] Implement wind forces.  
[ ] Fix wobbly behaviour.  
[ ] Run simulation in the GPU.  
