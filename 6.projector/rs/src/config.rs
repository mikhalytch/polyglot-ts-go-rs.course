use std::path::PathBuf;

use anyhow::{anyhow, Context, Ok, Result};

use crate::opts;

#[derive(Debug)]
pub struct Config {
    pub operation: Operation,
    pub pwd: PathBuf,
    pub config: PathBuf,
}

impl TryFrom<opts::Opts> for Config {
    type Error = anyhow::Error;

    fn try_from(value: opts::Opts) -> Result<Self> {
        Ok(Config {
            operation: value.args.try_into()?,
            pwd: get_pwd(value.pwd)?,
            config: get_config(value.config)?,
        })
    }
}

#[derive(Debug, PartialEq)]
pub enum Operation {
    Print(Option<String>),
    Add(String, String),
    Remove(String),
}

impl TryFrom<Vec<String>> for Operation {
    type Error = anyhow::Error;

    fn try_from(value: Vec<String>) -> Result<Self> {
        let mut value = value;

        if value.len() == 0 {
            return Ok(Operation::Print(None));
        }

        let term = value.get(0).expect("expected to exist at least single arg");

        if term == "add" {
            if value.len() != 3 {
                return Err(anyhow!("add expects 2 params, got {}", value.len() - 1));
            }

            let mut drain = value.drain(1..=2);

            return Ok(Operation::Add(
                drain.next().expect("expected"),
                drain.next().expect("expected"),
            ));
        }
        if term == "remove" {
            if value.len() != 2 {
                return Err(anyhow!("remove expects 2 params, got {}", value.len() - 1));
            }

            return Ok(Operation::Remove(value.pop().expect("expected")));
        }
        if value.len() > 1 {
            return Err(anyhow!(
                "print expects 0 or 1 argument, got {}",
                value.len()
            ));
        }
        Ok(Operation::Print(value.first().map(|x| x.to_owned())))
    }

    /* fn try_from(value: Vec<String>) -> Result<Self> {
        match value {
            empty if empty.is_empty() => Ok(Operation::Print(None)),
            _ => {
                let split_first = value.split_first();
                let head: Option<&String> = split_first.map(|(x, _)| x);
                let tail: Option<&[String]> = split_first.map(|(_, y)| y);
                match head
                    .expect("expected to have at least a single argument")
                    .as_str()
                {
                    "add" => match tail {
                        None => Err(anyhow!("add expects 2 arguments")),
                        Some(tail) if tail.len() != 2 => {
                            Err(anyhow!("add expects 2 arguments, got {}", tail.len()))
                        }
                        Some(tail) => Ok(Operation::Add(
                            tail.get(0) // next()
                                .expect("expected due to previous match case")
                                .to_owned(),
                            tail.get(1) // next()
                                .expect("expected due to previous match case")
                                .to_owned(),
                        )),
                    },
                    "remove" => match tail {
                        None => Err(anyhow!("rm expects 1 argument")),
                        Some(tail) if tail.len() != 1 => {
                            Err(anyhow!("rm expects 1 argument, got {}", tail.len()))
                        }
                        Some(tail) => Ok(Operation::Remove(
                            tail.get(0) // pop()
                                .expect("expected due to previous match case")
                                .to_owned(),
                        )),
                    },
                    _ if value.len() > 1 => Err(anyhow!("print expects 0 or 1 arguments, got {}", value.len())),
                    _ => Ok(Operation::Print(head.map(|x| x.to_owned()))),
                }
            }
        }
    } */
}

pub fn get_config(cfg: Option<PathBuf>) -> Result<PathBuf> {
    // 1. try cfg
    cfg.ok_or(anyhow!("expected config")) // ignore due to followup flow
        // 2. try XDG...
        .or_else(|_| {
            std::env::var("XDG_CONFIG_HOME")
                .context("expected XDG_CONFIG_HOME")
                .map(|loc| PathBuf::from(loc).join("projector").join("projector.json"))
        })
        // 3. try HOME
        .or_else(|_| {
            let my_home = homedir::get_my_home()?;
            my_home
                .ok_or(anyhow!("home is expected"))
                .map(|h| PathBuf::from(h).join(".projector").join("projector.json"))
        })
}

pub fn get_pwd(pwd: Option<PathBuf>) -> Result<PathBuf> {
    // 1. try cfg
    pwd.ok_or(anyhow!("expected pwd")) // ignored due to followup flow
        // 2. try env
        .or_else(|_| std::env::current_dir().map_err(|e| e.into()))
}

#[cfg(test)]
mod test {
    use anyhow::Result;

    use crate::{config::Operation, opts::Opts};

    use super::Config;

    #[test]
    fn test_print_all() -> Result<()> {
        let opts = Opts {
            args: vec![],
            config: None,
            pwd: None,
        };

        let config: Config = opts.try_into()?;

        assert_eq!(config.operation, Operation::Print(None));

        Ok(())
    }

    #[test]
    fn test_print_key() -> Result<()> {
        let opts = Opts {
            args: vec!["foo".to_string()],
            config: None,
            pwd: None,
        };

        let config: Config = opts.try_into()?;

        assert_eq!(config.operation, Operation::Print(Some("foo".to_owned())));

        Ok(())
    }

    #[test]
    fn test_print_error_multiple_keys() -> Result<()> {
        let opts = Opts {
            args: vec!["foo".into(), String::from("bar")],
            config: None,
            pwd: None,
        };

        let config: Result<Config> = opts.try_into();

        let got_err = config.map_err(|c| c.to_string());
        assert!(got_err.is_err());
        assert_eq!(got_err.unwrap_err(), "print expects 0 or 1 argument, got 2");

        Ok(())
    }

    #[test]
    fn test_add_key_value() -> Result<()> {
        let opts = Opts {
            args: vec!["add".to_string(), "foo".to_string(), "bar".to_string()],
            config: None,
            pwd: None,
        };

        let config: Config = opts.try_into()?;

        assert_eq!(
            config.operation,
            Operation::Add("foo".to_owned(), "bar".into())
        );

        Ok(())
    }

    #[test]
    fn remove_key() -> Result<()> {
        let opts = Opts {
            args: vec!["remove".to_string(), "foo".to_string()],
            config: None,
            pwd: None,
        };

        let config: Config = opts.try_into()?;

        assert_eq!(config.operation, Operation::Remove("foo".to_owned()));

        Ok(())
    }
}
