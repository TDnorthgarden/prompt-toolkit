use clap::{Parser};
use anyhow::Result;

pub struct Ps {
    pub format: String,
    pub container_id: String,
    pub ps_options: Vec<String>,
}

pub fn ps(args:Ps) -> Result<()> {
    Ok()
}