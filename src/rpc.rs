// Collection of functions to interface with ghostd.
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::error::Error;

#[derive(Debug, Clone, Default)]
pub struct AuthToken {
    user: String,
    password: String,
    url: String,
}

impl AuthToken {
    pub fn new() -> Self {
        Self {
            user: String::new(),
            password: String::new(),
            url: String::new(),
        }
    }
    pub fn target(mut self, ip: &str, port: u16, walletname: &str) -> Self {
        debug!("Generating auth ...");
        if walletname.len() == 0 {
            self.url = format!("http://{}:{}/", ip, port);
        } else {
            self.url = format!("http://{}:{}/wallet/{}", ip, port, walletname);
        }
        return self;
    }
    pub fn credentials(mut self, user: impl Into<String>, password: impl Into<String>) -> Self {
        trace!("Registering credentials ...");
        self.user = user.into();
        self.password = password.into();
        return self;
    }
}

fn parametrize(args: &str) -> Vec<Value> {
    trace!("Parsing arguments ...");
    let mut params: Vec<Value> = Vec::new();
    for entry in args.split(" ").collect::<Vec<&str>>() {
        match serde_json::from_str(entry) {
            Ok(val) => {
                params.push(val);
            }
            Err(_) => {
                params.push(Value::String(entry.to_string()));
            }
        }
    }
    return params;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RPCResponse {
    pub result: Value,
    pub error: Option<String>,
    pub id: String,
}

impl RPCResponse {
    fn unpack(self) -> Value {
        match self.error {
            Some(err) => {
                error!("{}", err);
                std::process::exit(1);
            }
            None => self.result,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Post<'r> {
    jsonrpc: &'r str,
    id: &'r str,
    method: Value,
    params: Value,
}

pub(crate) async fn call(args: &str, auth: &AuthToken) -> Result<Value, Box<dyn Error>> {
    let mut params = parametrize(args);
    let method = params[0].clone();
    params.remove(0);

    let post = Post {
        jsonrpc: "",
        id: "",
        method,
        params: Value::Array(params),
    };
    debug!("RPC: {} {} ...", &post.method, &post.params);
    let response = reqwest::Client::new()
        .post(auth.url.clone())
        .basic_auth(auth.user.clone(), Some(auth.password.clone()))
        .json(&post)
        .send()
        .await;
    match response {
        Ok(context) => {
            let rpcresponse: RPCResponse = context.json().await?;
            let json = rpcresponse.unpack();
            return Ok(json);
        }
        Err(err) => {
            error!("{}", err);
            std::process::exit(1);
        }
    }
}
