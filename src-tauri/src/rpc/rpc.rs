use std::{collections::HashMap, sync::Arc};

use jsonrpc_core::{IoHandler, Result};
use jsonrpc_derive::rpc;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::time::Instant;

use crate::{config::Config, tls};

macro_rules! call_aleo_function {
    ($func:ident($($arg:expr),*)) => {
        {
            let start_time = Instant::now();
            log::info!(target: "aleosdk","executing method '{}'",stringify!($func));
            let result  = aleowrap::$func($($arg),*);
            let elapsed_time = Instant::now() - start_time;
            log::info!(target: "aleosdk","method '{}' took {} ms", stringify!($func),elapsed_time.as_millis());
            result
        }
    };
}

lazy_static! {
    pub static ref RPC_HANDER: Arc<IoHandler> = Arc::new(init_rpc_hander());
}

pub fn init_rpc_hander() -> IoHandler {
    let mut io = jsonrpc_core::IoHandler::new();
    io.extend_with(super::rpc::RpcImpl.to_delegate());
    io
}

#[rpc]
pub trait Rpc {
    #[rpc(name = "deploy")]
    fn deploy(
        &self,
        private_key: String,
        program: String,
        fee_record: Option<String>,
        imports: Option<HashMap<String, String>>,
        priority_fee_in_microcredits: Option<u64>,
        query: Option<String>,
    ) -> Result<String>;

    #[rpc(name = "execute")]
    fn execute(
        &self,
        private_key: String,
        program_id: String,
        function: String,
        inputs: Vec<String>,
        record: Option<String>,
        fee: Option<u64>,
        query: Option<String>,
    ) -> Result<String>;

    #[rpc(name = "transfer")]
    fn transfer(
        &self,
        private_key: String,
        recipient: String,
        amount: u64,
        function: String,
        input_record: Option<String>,
        fee_record: Option<String>,
        fee: Option<u64>,
        query: Option<String>,
    ) -> Result<String>;

    #[rpc(name = "join")]
    fn join(
        &self,
        private_key: String,
        first_record: String,
        second_record: String,
        fee_record: Option<String>,
        fee: Option<u64>,
        query: Option<String>,
    ) -> Result<String>;

    #[rpc(name = "split")]
    fn split(
        &self,
        private_key: String,
        record: String,
        amount: u64,
        query: Option<String>,
    ) -> Result<String>;

    #[rpc(name = "deployment_cost")]
    fn deployment_cost(
        &self,
        program: String,
        imports: Option<HashMap<String, String>>,
    ) -> Result<String>;

    #[rpc(name = "execution_cost")]
    fn execution_cost(
        &self,
        private_key: String,
        program_id: String,
        function: String,
        inputs: Vec<String>,
        query: Option<String>,
    ) -> Result<String>;

    #[rpc(name = "decrypt_records")]
    fn decrypt_records(&self, private_key: String, records: Vec<String>) -> Result<Vec<String>>;

    #[rpc(name = "discovery")]
    fn discovery(&self) -> Result<Discovery>;
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Discovery {
    version: String,
    features: Vec<String>,
    pubkey: String,
}

pub struct RpcImpl;

impl Rpc for RpcImpl {
    fn deploy(
        &self,
        private_key: String,
        program: String,
        fee_record: Option<String>,
        imports: Option<HashMap<String, String>>,
        priority_fee_in_microcredits: Option<u64>,
        query: Option<String>,
    ) -> Result<String> {
        log::info!(target: "rpc","executing rpc method 'deploy'");
        call_aleo_function!(deploy(
            &private_key,
            &program,
            fee_record.as_deref(),
            imports,
            priority_fee_in_microcredits,
            query.as_deref()
        ))
        .to_jsonrpc_result()
        .log_rpc_error("deploy")
    }

    fn execute(
        &self,
        private_key: String,
        program_id: String,
        function: String,
        inputs: Vec<String>,
        record: Option<String>,
        fee: Option<u64>,
        query: Option<String>,
    ) -> Result<String> {
        log::info!(target: "rpc","executing rpc method 'execute'");
        call_aleo_function!(execute(
            &private_key,
            &program_id,
            &function,
            inputs,
            record.as_deref(),
            fee,
            query.as_deref()
        ))
        .to_jsonrpc_result()
        .log_rpc_error("execute")
    }

    fn transfer(
        &self,
        private_key: String,
        recipient: String,
        amount: u64,
        function: String,
        input_record: Option<String>,
        fee_record: Option<String>,
        fee: Option<u64>,
        query: Option<String>,
    ) -> Result<String> {
        log::info!(target: "rpc","executing rpc method 'transfer'");
        call_aleo_function!(transfer(
            &private_key,
            &recipient,
            amount,
            &function,
            input_record.as_deref(),
            fee_record.as_deref(),
            fee,
            query.as_deref()
        ))
        .to_jsonrpc_result()
        .log_rpc_error("transfer")
    }

    fn join(
        &self,
        private_key: String,
        first_record: String,
        second_record: String,
        fee_record: Option<String>,
        fee: Option<u64>,
        query: Option<String>,
    ) -> Result<String> {
        log::info!(target: "rpc","executing rpc method 'join'");
        call_aleo_function!(join(
            &private_key,
            &first_record,
            &second_record,
            fee_record.as_deref(),
            fee,
            query.as_deref()
        ))
        .to_jsonrpc_result()
        .log_rpc_error("join")
    }

    fn split(
        &self,
        private_key: String,
        record: String,
        amount: u64,
        query: Option<String>,
    ) -> Result<String> {
        log::info!(target: "rpc","executing rpc method 'split'");
        call_aleo_function!(split(&private_key, &record, amount, query.as_deref()))
            .to_jsonrpc_result()
            .log_rpc_error("split")
    }

    fn deployment_cost(
        &self,
        program: String,
        imports: Option<HashMap<String, String>>,
    ) -> Result<String> {
        log::info!(target: "rpc","executing rpc method 'deployment_cost'");
        call_aleo_function!(deployment_cost(&program, imports))
            .to_jsonrpc_result()
            .log_rpc_error("deployment_cost")
    }

    fn execution_cost(
        &self,
        private_key: String,
        program_id: String,
        function: String,
        inputs: Vec<String>,
        query: Option<String>,
    ) -> Result<String> {
        log::info!(target: "rpc","executing rpc method 'execution_cost'");
        call_aleo_function!(execution_cost(
            &private_key,
            &program_id,
            &function,
            inputs,
            query.as_deref()
        ))
        .to_jsonrpc_result()
        .log_rpc_error("execution_cost")
    }

    fn decrypt_records(&self, private_key: String, records: Vec<String>) -> Result<Vec<String>> {
        log::info!(target: "rpc","executing rpc method 'decrypt_records'");
        call_aleo_function!(decrypt_records(&private_key, records))
            .to_jsonrpc_result()
            .log_rpc_error("decrypt_records")
    }

    fn discovery(&self) -> Result<Discovery> {
        log::info!(target: "rpc","executing rpc method 'discovery'");
        let client_secret = Config::get_config().get_secret_key().to_jsonrpc_result()?;
        Ok(Discovery {
            version: env!("CARGO_PKG_VERSION").to_string(),
            features: vec![
                "deploy".to_string(),
                "execute".to_string(),
                "transfer".to_string(),
                "join".to_string(),
                "split".to_string(),
                "deployment_cost".to_string(),
                "execution_cost".to_string(),
                "decrypt_records".to_string(),
            ],
            pubkey: hex::encode(tls::get_p256_pubkey(&client_secret)),
        })
    }
}

pub fn to_jsonrpc_error(err: anyhow::Error) -> jsonrpc_core::error::Error {
    let mut error = jsonrpc_core::error::Error::new(jsonrpc_core::ErrorCode::ServerError(500));
    error.data = Some(serde_json::Value::String(format!("{:#?}", err)));
    error.message = err.to_string();
    error
}

trait ToJsonRpcResult<T> {
    fn to_jsonrpc_result(self) -> jsonrpc_core::Result<T>;
}

impl<T> ToJsonRpcResult<T> for anyhow::Result<T> {
    fn to_jsonrpc_result(self) -> jsonrpc_core::Result<T> {
        match self {
            Ok(v) => Ok(v),
            Err(err) => {
                let error = to_jsonrpc_error(err);
                Err(error)
            }
        }
    }
}

trait RpcLog<T> {
    fn log_rpc_error(self, method: &str) -> jsonrpc_core::Result<T>;
}

impl<T> RpcLog<T> for jsonrpc_core::Result<T> {
    fn log_rpc_error(self, method: &str) -> jsonrpc_core::Result<T> {
        if self.is_err() {
            let err = self.as_ref().err().unwrap().clone();
            log::error!(target: "rpc error", "method: {} ,code:{}, msg: {}",method, err.code.description(), err.message);

            if let Some(value) = err.data {
                if value.is_string() {
                    log::error!(target: "rpc error","{}",value.as_str().unwrap())
                }
            }
        }
        self
    }
}
