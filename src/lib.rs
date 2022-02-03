use pyo3::prelude::*;
use rand::seq::SliceRandom;
use rand::thread_rng;

#[pyfunction]
#[pyo3(name = "poyo")]
fn poyo_() -> PyResult<String> {
    Ok("poyo".to_string())
}

#[pyfunction]
fn sushi() -> PyResult<String> {
    Ok("sushi".to_string())
}

#[pyfunction]
fn sushi_or_poyo() -> PyResult<String> {
    let choices = ["poyo", "sushi"];
    let mut rng = thread_rng();
    Ok(choices.choose(&mut rng).unwrap().to_string())
}

#[pyfunction]
fn call_from_poyo(fun: &PyAny) -> PyResult<&PyAny> {
    fun.call1(("poyo".to_string(),))
}

#[pymodule]
fn poyo(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(poyo_, m)?)?;
    m.add_function(wrap_pyfunction!(sushi, m)?)?;
    m.add_function(wrap_pyfunction!(sushi_or_poyo, m)?)?;
    m.add_function(wrap_pyfunction!(call_from_poyo, m)?)?;
    Ok(())
}
