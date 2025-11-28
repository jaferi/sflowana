use std::env;

pub struct Config {
    pub rpc_url: String,
}

impl Config {
    pub fn from_env() -> Result<Self, env::VarError> {
        let rpc_url = env::var("SFLOWANA_RPC_URL")?;

        Ok(Self { rpc_url })
    }
}
