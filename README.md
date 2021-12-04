# A projected circle packing

As the mouse pointer is moved, a rectangle of constant area is drawn (in yellow), aligned with the line from the origin to the mouse pointer. Notice that tracing the edge of a red circle produces movement in a corner of the rectangle which traces out a green circle. Notice that, conversely, tracing the edge of a green circle produces a the edge of a red circle.

The blue circles are produced by tracing out the horizontal white lines.

Use cursor keys to move the view, + and - to zoom in and out. Press space to toggle animation.

Inspired by the constant area method from https://youtu.be/hSsRcpIsunk?t=167

# Dev

Literally, my first Rust program - so a total mess and not very idiomatic.

Run locally with `cargo run`

Build for web with `make`. Run with webserver using `make run`

