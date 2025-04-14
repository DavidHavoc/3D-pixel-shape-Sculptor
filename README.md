# 3D Shape Sculptor (Rust + Bevy)

This application generates a 3D voxel representation of various geometric shapes based on user-defined dimensions. It uses the Bevy game engine for rendering and `egui` for the user interface.

## Features

* Selectable geometric shapes: Cube, Sphere, Cylinder, Cone, Square Pyramid.
* Adjustable dimensions (Width, Depth, Height) from 1 to 32 units.
* Interactive 3D view with orbit camera controls (rotate, pan, zoom).
* Voxel color: Pink (#AC1754)
* Background color: Black

## Prerequisites

* **Rust Toolchain:** Ensure you have Rust and Cargo installed. If not, follow the instructions at [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).
* **Build Tools:**
    * **Windows:** You might need the "Desktop development with C++" workload installed via the Visual Studio Installer.
    * **Linux:** You'll need development packages like `build-essential`, `libxkbcommon-dev`, `libudev-dev`, `libwayland-dev`, `libxrandr-dev`, `libxcb-shape0-dev`, `libxcb-xfixes0-dev` (package names may vary slightly based on your distribution - e.g., `pkg-config`, `libudev-devel` on Fedora). You also need development libraries for audio (like `libasound2-dev` or `alsa-lib-devel`).
    * **macOS:** You need Xcode Command Line Tools (`xcode-select --install`).
* **Graphics Drivers:** Ensure you have up-to-date graphics drivers supporting Vulkan (preferred), Metal (macOS), or DirectX 12 (Windows).

## How to Build and Run

1.  **Clone or Download:** Get the project code onto your local machine.
2.  **Navigate to Project Directory:** Open your terminal or command prompt and change into the project's root directory (`shape_sculptor`).
    ```bash
    cd path/to/shape_sculptor
    ```
3.  **Build the Project:** Compile the code using Cargo. This might take a few minutes the first time as it downloads and compiles dependencies.
    ```bash
    cargo build
    ```
    For potentially better performance, build in release mode (takes longer to compile):
    ```bash
    cargo build --release
    ```
4.  **Run the Application:**
    * **Development Mode:**
        ```bash
        cargo run
        ```
    * **Release Mode (if built with `--release`):**
        ```bash
        cargo run --release
        ```
    Alternatively, you can run the executable directly after building:
    * **UNIX (Linux/macOS):** `./target/debug/shape_sculptor` or `./target/release/shape_sculptor`
    * **Windows:** `.\target\debug\shape_sculptor.exe` or `.\target\release\shape_sculptor.exe`

## Controls

* **Rotate View:** Hold the **Left Mouse Button** and drag.
* **Pan View:** Hold the **Right Mouse Button** and drag.
* **Zoom View:** Use the **Mouse Scroll Wheel**.
* **Adjust Dimensions/Shape:** Use the sliders and dropdown menu in the "Sculptor Controls" window. The shape will update automatically.