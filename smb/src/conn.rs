use log::info;
use pyo3::prelude::*;
use core::result::Result;
use rayon::prelude::*;
use std::io::{Write};
use pavao::{SmbClient, SmbCredentials, SmbMode, SmbOpenOptions, SmbOptions};
use py_types::{SMBError, SMBErrorType};
use read_byte_slice::{ByteSliceIter, FallibleStreamingIterator};
use std::fs::File;


#[pyclass]
//#[derive(Debug, Clone)]
pub struct Connection {
    //conn: QStream,
    #[pyo3(get)]
    hostname: String,
    #[pyo3(get)]
    username: String,
    password: String,
    #[pyo3(get)]
    port: Option<u16>,
    client: Option<SmbClient>,
}


impl Connection {

    pub fn new(hostname: String, 
                username: String, 
                password: String, 
                port: Option<u16>) -> Result<Connection, SMBError> {
        Ok(Self {   hostname,
                    username,
                    password,
                    port,
                    client:None
                }
            )
    }
}
#[pymethods]
impl Connection {
    pub fn connect(&self, py: Python) ->  PyResult<Py<PyAny>> {
        let base_url = format!("smb://{0}", self.hostname);
        
        let full_url = match self.port {
            Some(port) => format!("{base_url}:{port}" ),
            None => base_url
        };

        let client = SmbClient::new(
            SmbCredentials::default()
                .server(full_url)
                .password(self.password.clone())
                .username(self.username.clone()),    
            SmbOptions::default().one_share_per_server(false),
        );

        match client {
            Ok(c) => {
                
                let conn =  Connection{
                        hostname : self.hostname.clone(),
                        username : self.username.clone(),
                        password : self.password.clone(),
                        port: self.port,
                        client: Some(c)
                    };
                Ok(conn.into_py(py))
                
            }
            Err(e) => {
                
                let error = SMBError {
                    code: String::from("Error Connecting"),
                    message: format!("Error Connecting {}", e ),
                    error: SMBErrorType::ConnectionError
                };
                Err(error.to_pyerr())
            }
        }

    }

    pub fn write_file(&self, py: Python, source_file_path: String, target_file_path: String) ->  PyResult<Py<PyAny>> {


        match &self.client {
            Some(client) => { 

                let target_file = &mut client.open_with(target_file_path, 
                                                                    SmbOpenOptions::default()
                                                                            .create(true)
                                                                            .write(true)
                                                                    );
                match target_file {
                    Ok(file) => {
                        let source_file = File::open(source_file_path);
                        match source_file {
                            Ok(src_file) => {
                                let mut iter = ByteSliceIter::new(src_file, 1000000);
                                while let Some(chunk) = iter.next()? {
                                    info!("Got chunk");
                                    let result = file.write(chunk);

                                }
                                Ok(true.into_py(py))
                            }
                            Err(e) => {
                                let error = SMBError {
                                    code: String::from("Error writing file"),
                                    message: format!("Error opening target file on smb server {}", e),
                                    error: SMBErrorType::MkdirError
                                };
                                Err(error.to_pyerr())
                            }
                        }

                    }
                    Err(e) => {
                        let error = SMBError {
                            code: String::from("Error writing file"),
                            message: format!("Error opening target file on smb server {}", e),
                            error: SMBErrorType::MkdirError
                        };
                        Err(error.to_pyerr())
                    }
                }

            }
            None => { 
                let error = SMBError {
                code: String::from("Error - No Client / Connection"),
                message: String::from("No Client / Connection"),
                error: SMBErrorType::ConnectionError
                };
                Err(error.to_pyerr())
            }
        }
    }

    pub fn mkdir(&self, py: Python, source_path: String) ->  PyResult<Py<PyAny>> {
        match &self.client {
            Some(client) => { 

                let result = client.mkdir(source_path, SmbMode::from(0o755));
                match result {
                    Ok(_) => {
                        Ok(true.into_py(py))
                    }
                    Err(e) => {
                        let error = SMBError {
                            code: String::from("Error"),
                            message: format!("Error Making Directory {}", e),
                            error: SMBErrorType::MkdirError
                        };
                        Err(error.to_pyerr())
                    }
                }
            }
            None => { 
                let error = SMBError {
                code: String::from("Error - No Client / Connection"),
                message: String::from("No Client / Connection"),
                error: SMBErrorType::ConnectionError
                };
                Err(error.to_pyerr())
            }
        }

    }

    pub fn list_dir(&self, path: String) -> PyResult<Vec<String>> {


        let tmp_path = path.clone();
        match &self.client {
                Some(client) => {
                    match client.list_dir(tmp_path) {
                        Ok(v) => {
                            let names = v.into_iter().map(|x| {
                                String::from(x.name())
                            }).collect::<Vec<String>>();
                            Ok(names)
                        }
                        Err(e) => {
                            let err_path  = path.clone();
                            let error = SMBError {
                                code: String::from("Error"),
                                message: format!("Error listing path {err_path} - {}", e),
                                error: SMBErrorType::ConnectionError
                            };
                            Err(error.to_pyerr())
                        }
                    }
                }
                None => { 
                        let error = SMBError {
                        code: String::from("Error"),
                        message: String::from("No Client / Connection"),
                        error: SMBErrorType::ConnectionError
                    };
                    Err(error.to_pyerr())
                }

        }
    }

}