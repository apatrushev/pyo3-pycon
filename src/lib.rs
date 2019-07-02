use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyclass]
struct MyClass{
    #[pyo3(get, set)]
        num: usize,
        #[pyo3(get, set)]
        val: String,

}

#[pyfunction]
fn sum_as_str(a: usize, b: usize) -> MyClass {
    MyClass {
        num: (a + b),
        val: (a+b).to_string()
    }
}

#[pymodule]
fn {{crate_name}}(_: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(sum_as_str))?;
    Ok(())
}
