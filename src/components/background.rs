use dioxus::prelude::*;

#[cfg(target_arch = "wasm32")]
const RESIZE_POLL_MS: u32 = 250;

fn viewport_size() -> (f64, f64) {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(win) = web_sys::window() {
            let width = win
                .inner_width()
                .ok()
                .and_then(|v| v.as_f64())
                .unwrap_or(1280.0);
            let height = win
                .inner_height()
                .ok()
                .and_then(|v| v.as_f64())
                .unwrap_or(720.0);
            return (width, height);
        }
    }
    (1280.0, 720.0)
}

fn hex_metrics(width: f64) -> (f64, f64) {
    if width <= 480.0 {
        (16.0, 3.0)
    } else if width <= 768.0 {
        (20.0, 4.0)
    } else {
        (36.0, 6.0)
    }
}

fn compute_grid_size(width: f64, height: f64, size: f64, gap: f64) -> (usize, usize) {
    let col_step = size * 1.5 + gap;
    let row_step = size * 1.732050808 + gap;

    let padded_width = width + size * 4.0;
    let padded_height = height + size * 4.0;

    let cols = (padded_width / col_step).ceil() as usize + 2;
    let rows = (padded_height / row_step).ceil() as usize + 2;

    (rows.max(1), cols.max(1))
}

// Generate flat-top hexagon path centered at origin.
fn hex_path(size: f64) -> String {
    let r = size;
    let h = r * 0.866025403784; // √3/2
    format!(
        "M {},{} L {},{} L {},{} L {},{} L {},{} L {},{} Z",
        r, 0.0,
        r / 2.0, h,
        -r / 2.0, h,
        -r, 0.0,
        -r / 2.0, -h,
        r / 2.0, -h
    )
}

#[component]
pub fn Background() -> Element {
    #[allow(unused_mut)]
    let mut dimensions = use_signal(|| {
        let (w, h) = viewport_size();
        let (size, gap) = hex_metrics(w);
        let (rows, cols) = compute_grid_size(w, h, size, gap);
        (size, gap, rows, cols)
    });

    #[cfg(target_arch = "wasm32")]
    {
        use_future(move || async move {
            let mut last = viewport_size();

            loop {
                gloo_timers::future::TimeoutFuture::new(RESIZE_POLL_MS).await;

                let current = viewport_size();
                let width_changed = (current.0 - last.0).abs() > 1.0;
                let height_changed = (current.1 - last.1).abs() > 1.0;

                if width_changed || height_changed {
                    last = current;
                    let (size, gap) = hex_metrics(current.0);
                    let (rows, cols) = compute_grid_size(current.0, current.1, size, gap);
                    dimensions.set((size, gap, rows, cols));
                }
            }
        });
    }

    let (hex_size, hex_gap, rows, cols) = dimensions();

    let col_spacing = hex_size * 1.5 + hex_gap;
    let row_spacing = hex_size * 1.732050808 + hex_gap;

    let hex_d = hex_path(hex_size);
    let base_x = -2.0 * hex_size;
    let base_y = -2.0 * hex_size;

    rsx! {
        div { id: "hex-background",
            svg {
                width: "100%",
                height: "100%",
                xmlns: "http://www.w3.org/2000/svg",
                for c in 0..cols {
                    {
                        let x = base_x + (c as f64) * col_spacing;
                        let y_col_offset = if c % 2 == 0 { 0.0 } else { row_spacing * 0.5 };
                        rsx! {
                            for r in 0..rows {
                                {
                                    let y = base_y + (r as f64) * row_spacing + y_col_offset;
                                    rsx! {
                                        path {
                                            d: "{hex_d}",
                                            transform: "translate({x}, {y})",
                                            fill: "#0c0c0c",

                                            stroke_width: "1",
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
