use std::{collections::HashMap, sync::Arc};

use pyo3::{prelude::*, types::PyDict};
use pyo3_asyncio::tokio::future_into_py_with_locals;
use reccaster::{Record, Reccaster};
use tokio::sync::Mutex;
       
#[pyclass]
pub struct PyRecord(Record);

#[pymethods]
impl PyRecord {
    #[new]
    #[pyo3(signature = (name, r#type, alias=None, properties=HashMap::new()))]
    fn new(name: String, r#type: String, alias: Option<String>, properties: HashMap<String, String>) -> Self {
        PyRecord(Record { name, r#type, alias, properties })
    }

    #[getter]
    fn name(&self) -> &str {
        &self.0.name
    }

    #[getter]
    fn r#type(&self) -> &str {
        &self.0.r#type
    }

    #[getter]
    fn alias(&self) -> Option<&String> {
        self.0.alias.as_ref()
    }

    #[getter]
    fn properties<'py>(&self, py: Python<'py>) -> PyResult<&'py PyDict> {
        let properties = PyDict::new(py);
        for (key, value) in &self.0.properties {
            properties.set_item(key, value)?;
        }
        Ok(properties)
    }

}

impl<'source> FromPyObject<'source> for PyRecord {
    fn extract(ob: &'source PyAny) -> PyResult<Self> {
        let name: String = ob.getattr("name")?.extract().unwrap_or_else(|_| "OPS no name !!!!!!!!!!!".to_string());
        let r#type: String = ob.getattr("type")?.extract()?;
        let alias: Option<String> = ob.getattr("alias")?.extract()?;
        let properties: HashMap<String, String> = ob.getattr("properties")?.extract()?;
        
        Ok(PyRecord (Record { name, r#type, alias, properties }))
    }
}

#[pyclass]
struct PyReccaster {
    reccaster: Arc<Mutex<Reccaster>>,
}

#[pymethods]
impl PyReccaster {

    #[staticmethod]
    fn setup(py: Python, records: Vec<PyRecord>) -> PyResult<&PyAny> {
        let locals = pyo3_asyncio::tokio::get_current_locals(py)?;
        let pvs = records.iter().map(|record: &PyRecord| record.0.clone()).collect::<Vec<Record>>();
        future_into_py_with_locals(py, locals.clone(), async move {
            let recc = Reccaster::new(pvs).await;
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
    m.add_class::<PyRecord>()?;
    Ok(())
}
