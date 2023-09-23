//-------------------------------------------------------------------------------------------------
// Functions
//-------------------------------------------------------------------------------------------------

use std::path::PathBuf;

pub enum Env {
    Dev,
    Staging,
    Test,
    Prod,
}

//-------------------------------------------------------------------------------------------------
// Functions
//-------------------------------------------------------------------------------------------------

pub fn load_env(env: Env) -> Option<PathBuf> {
    match env {
        Env::Dev => dotenv::from_filename(".env.dev").ok(),
        Env::Staging => dotenv::from_filename(".env.staging").ok(),
        Env::Test => dotenv::from_filename(".env.test").ok(),
        Env::Prod => dotenv::dotenv().ok(),
    }
}
