use dioxus::prelude::*;

// A simple 2D grid of coordinates representing the hexagon positions.
// We keep it as a Vec<Vec<(row, col)>> to make it easy to reason about rows and columns.
pub type HexCoord = (usize, usize);

fn generate_hex_coords(rows: usize, cols: usize) -> Vec<Vec<HexCoord>> {
    let mut grid = Vec::with_capacity(rows);
    for r in 0..rows {
        let mut row = Vec::with_capacity(cols);
        for c in 0..cols {
            row.push((r, c));
        }
        grid.push(row);
    }
    grid
}

#[component]
pub fn Background() -> Element {
    // Choose a grid large enough to fill the viewport; CSS will clip overflow.
    // Later we can compute this responsively from the viewport size.
    let rows = 24usize;
    let cols = 48usize;
    let grid = generate_hex_coords(rows, cols);

    rsx! {
        // Fixed, full-viewport container for the background
        div {
            id: "hex-background",
            // Render one hex per grid cell. Store row/col as CSS variables.
            // For flat-top hexes, we offset ODD COLUMNS vertically in CSS for honeycomb interlock.
            for (r, row) in grid.iter().enumerate() {
                for (c, _) in row.iter().enumerate() {
                    div {
                        class: "hex",
                        "data-parity": if c % 2 == 0 { "even" } else { "odd" },
                        style: format!("--row: {}; --col: {};", r, c),
                    }
                }
            }
        }
    }
}
