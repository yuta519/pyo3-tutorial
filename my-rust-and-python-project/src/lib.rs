use pyo3::prelude::*;

#[pyfunction]
fn print_something() {
    println!("You are my special");
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn my_rust_lib(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(print_something, m)?)?;

    Ok(())
}
