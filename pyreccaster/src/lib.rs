use std::sync::Arc;

use pyo3::prelude::*;
use pyo3_asyncio::tokio::future_into_py_with_locals;
use reccaster::Reccaster;
use tokio::sync::Mutex;

#[pyclass]
struct PyReccaster {
    reccaster: Arc<Mutex<Reccaster>>
}


#[pymethods]
impl PyReccaster {

    #[staticmethod]
    fn setup(py: Python) -> PyResult<&PyAny> {
        let locals = pyo3_asyncio::tokio::get_current_locals(py)?;
        
        future_into_py_with_locals(py, locals.clone(), async move {
            let recc = Reccaster::new().await;
            let pyrecc = PyReccaster { reccaster: Arc::new(Mutex::new(recc)) };
            Python::with_gil(|py| Ok(pyrecc.into_py(py)))
        })
    }

    fn run<'p>(&self, py: Python<'p>) -> PyResult<&'p PyAny> {
        let recc_arc = self.reccaster.clone();
        let locals = pyo3_asyncio::tokio::get_current_locals(py)?;

        future_into_py_with_locals(py, locals.clone(), async move {
            let mut recc = recc_arc.lock().await;
            recc.run().await;
            Ok(())
        })
    }
}

#[pymodule]
fn pyreccaster(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyReccaster>()?;
    Ok(())
}
