use std::fmt::Display;

use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum SMBErrorType {
    ConnectionError,
    MkdirError,
    WriteError,
}

#[pyclass(name = "SMBError", extends = PyTypeError)]
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct SMBError {
    pub code: String,
    pub message: String,
    pub error: SMBErrorType,

}

impl ToPyObject for SMBErrorType {
    fn to_object(&self, py: Python) -> PyObject {
        self.to_string().to_object(py)
    }
}

impl<'a> FromPyObject<'a> for SMBErrorType {
    fn extract(ob: &PyAny) -> PyResult<Self> {
        let s = ob.extract::<String>()?;
        match s.as_str() {
            "ConnectionError" => Ok(SMBErrorType::ConnectionError),
            "MkdirError" => Ok(SMBErrorType::MkdirError),
            "WriteError" => Ok(SMBErrorType::WriteError),
            _ => Err(PyTypeError::new_err(format!(
                "Cannot convert {} to DBError",
                s
            ))),
        }
    }
}

impl Display for SMBErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v = match self {
            SMBErrorType::ConnectionError => "ConnectionError".to_string(),
            SMBErrorType::MkdirError => "MkdirError".to_string(),
            SMBErrorType::WriteError => "WriteError".to_string(),
        };
        write!(f, "{}", v)
    }
}

#[pymethods]
impl SMBError {
    #[new]
    pub fn py_new(code: String, message: String, error: SMBErrorType ) -> Self {
        Self {
            code,
            message,
            error,
        }
    }

    pub fn __str__(&self) -> String {
        format!(
            "SMBError(code='{}', message='{}')",
            self.code, self.message
        )
    }

    pub fn __repr__(&self) -> String {
        self.__str__()
    }

    pub fn code(&self) -> String {
        self.code.clone()
    }

    pub fn message(&self) -> String {
        self.message.clone()
    }

    pub fn error(&self) -> String {
        self.error.to_string()
    }

    pub fn to_pyerr(&self) -> PyErr {
        PyErr::new::<SMBError, _>((
            self.code.clone(),
            self.message.clone(),
            self.error.to_string(),
        ))
    }
}

pub fn py_error(err: String, typ: SMBErrorType) -> SMBError {
    SMBError::py_new(String::from("0"), err, typ)
}
