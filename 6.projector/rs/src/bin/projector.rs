use anyhow::Result;
use clap::Parser;
use rs::{
    config::{self, Operation},
    opts::Opts,
    projector::Projector,
};

fn main() -> Result<()> {
    let opts = Opts::parse();
    // println!("{:?}", opts)

    // let operation: Operation = opts.args.try_into()?;
    // let config = config::get_config(opts.config)?;
    // let pwd = config::get_pwd(opts.pwd)?;
    // println!("op: {:?}; config: {:?}; pwd: {:?}", operation, config, pwd);
    let conf = config::Config::try_from(opts)?;
    // println!("{:?}", config);

    let mut proj = Projector::from_config(conf.config, conf.pwd);

    match conf.operation {
        Operation::Print(None) => {
            let value = proj.get_value_all();
            let value = serde_json::to_string(&value)?;
            println!("{}", value);
        }
        Operation::Print(Some(k)) => {
            let value = proj.get_value(&k);
            // if let Some(value) = value {
            //     println!("{}", value);
            // }
            value.iter().for_each(|v| println!("{}", v));
        }
        Operation::Add(k, v) => {
            proj.set_value(k.to_string(), v.to_string());
            proj.save()?;
        }
        Operation::Remove(k) => {
            proj.remove_value(&k);
            proj.save()?;
        }
    }
    Ok(())
}
