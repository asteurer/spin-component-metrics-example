use spin_sdk::http::{IntoResponse, Request, Response};
use spin_sdk::http_component;

/// A simple Spin HTTP component.
#[http_component]
fn handle_component_metrics_example(_req: Request) -> anyhow::Result<impl IntoResponse> {
    fibonacci_recursive(43);
    allocate_memory(50);
    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body("Hello, world!")
        .build())
}

/// Generates CPU usage
fn fibonacci_recursive(n: u32) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2),
    }
}

/// Allocate {mb} Megabytes of memory
fn allocate_memory(mb: usize) {
    let mut buffer: Vec<u8> = vec![0; mb * 1024 * 1024];

    for (i, byte) in buffer.iter_mut().enumerate() {
        *byte = (i % 256) as u8;
    }

    // Prevent the compiler from optimizing away the buffer allocations.
    std::hint::black_box(buffer);
}
