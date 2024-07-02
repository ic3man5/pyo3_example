use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn pyo3_example(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_as_string() {
        pyo3::append_to_inittab!(pyo3_example);
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| Python::run_bound(py, 
            "import pyo3_example;\nif pyo3_example.sum_as_string(1, 2) != '3': raise ValueError('Wrong sum')", None, None)).expect("Failed to run python module");
    }
}
