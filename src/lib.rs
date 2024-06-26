use smb::Connection;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

pub fn get_version() -> String {
    let version = env!("CARGO_PKG_VERSION").to_string();
    // cargo uses "1.0-alpha1" etc. while python uses "1.0.0a1", this is not full compatibility,
    // but it's good enough for now
    // see https://docs.rs/semver/1.0.9/semver/struct.Version.html#method.parse for rust spec
    // see https://peps.python.org/pep-0440/ for python spec
    // it seems the dot after "alpha/beta" e.g. "-alpha.1" is not necessary, hence why this works
    version.replace("-alpha", "a").replace("-beta", "b")
}

#[pyfunction]
fn new(py: Python, 
    hostname: String, 
    username: String,
    password: String,
    port: Option<u16>) -> PyResult<Py<PyAny>> {
    
    
    match Connection::new(hostname, username, password, port) {
        Ok(r) => Ok(r.into_py(py)),
        Err(e) => Err(e.to_pyerr()),
    }

    /*pyo3_asyncio::tokio::future_into_py(py, async move {
        match Connection::new(hostname, port).await {
            Ok(r) => Ok(r),
            Err(e) => Err(e.to_pyerr()),
        }
    })
    */
}

#[pymodule]
fn fastsmb(_py: Python, m: &PyModule) -> PyResult<()> {
    pyo3_log::init();
    m.add("__version__", get_version())?;
    m.add_function(wrap_pyfunction!(new, m)?)?;
    m.add_class::<Connection>()?;
    //m.add_class::<PySQLXResult>()?;
    //m.add_class::<PySQLXError>()?;
    Ok(())
}