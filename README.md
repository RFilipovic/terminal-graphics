Terminal Graphics Renderer
A lightweight 3D wireframe renderer that runs entirely in the terminal, written in Rust. Loads 3D objects from a simple text-based format defining vertices and edges, applies perspective projection, and renders them as ASCII art directly in your terminal window. Supports real-time rotation on all three axes (X, Y, Z) via keyboard input using raw terminal mode.
Features:

  - Custom .txt object format for defining 3D meshes
  - Perspective projection from 3D to 2D
  - Real-time keyboard-driven rotation
  - ASCII framebuffer rendering
  - Built with crossterm for cross-platform terminal control
