use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Data {
    pub projector: HashMap<PathBuf, HashMap<String, String>>,
}

pub struct Projector {
    config: PathBuf,
    pwd: PathBuf,
    data: Data,
}

impl Projector {
    pub fn new(config: PathBuf, pwd: PathBuf, data: Data) -> Self {
        Self { config, pwd, data }
    }
    pub fn default(config: PathBuf, pwd: PathBuf) -> Self {
        Self::new(config, pwd, Data::default())
    }
    pub fn from_config(config: PathBuf, pwd: PathBuf) -> Self {
        // Get config file
        // Deserialize
        // use it
        // else return default projector

        let data = fs::read_to_string(&config)
            .and_then(|contents| serde_json::from_str::<Data>(&contents).map_err(|e| e.into()))
            .unwrap_or_else(|_| Data::default()); // ignore errors, just provide default value

        Self::new(config, pwd, data)
    }

    pub fn save(&self) -> Result<()> {
        self.config
            .parent()
            .map(|pdir| fs::create_dir_all(pdir))
            .unwrap_or_else(|| Ok(()))?;

        let contents = serde_json::to_string(&self.data)?;
        fs::write(&self.config, contents)?;
        Ok(())
    }

    // -----

    pub fn get_value_all(&self) -> HashMap<&String, &String> {
        self.pwd
            .as_path()
            .ancestors() // paths from pwd to '/'
            .collect::<Vec<&Path>>()
            .iter()
            .rev() // from '/' to pwd
            .fold(HashMap::new(), |mut result_acc, &path| {
                self.data
                    .projector
                    .get(path)
                    .inspect(|&path_data| result_acc.extend(path_data));
                result_acc
            })
    }

    pub fn get_value(&self, key: &str) -> Option<&String> {
        self.pwd.as_path().ancestors().find_map(|path| {
            self.data
                .projector
                .get(&path.to_path_buf())
                .and_then(|path_data| path_data.get(key))
        })
    }

    pub fn set_value(&mut self, key: String, value: String) {
        self.data
            .projector
            .entry(self.pwd.clone())
            .or_default()
            .insert(key, value);
    }

    pub fn remove_value(&mut self, key: &str) {
        self.data
            .projector
            .get_mut(&self.pwd)
            // .iter_mut() // required to for_each
            // ----- map is not totally correct, since we don't transform, but rather do the thing to internals
            // .for_each(|pwd_data| {
            .map(|pwd_data| {
                pwd_data.remove(key);
            });
    }
}

#[cfg(test)]
mod test {
    use std::{collections::HashMap, path::PathBuf};

    use collection_macros::hashmap;

    use super::{Data, Projector};

    fn get_data() -> HashMap<PathBuf, HashMap<String, String>> {
        hashmap! {
            PathBuf::from( "/" )=> hashmap! {
                "foo".into()=> "bar1".into(),
                "fem".into()=> "is great".into(),
            },
            PathBuf::from("/foo")=> hashmap! { "foo".into()=> "bar2".into() },
            PathBuf::from("/foo/bar")=> hashmap! { "foo".into()=> "bar3".into() },
        }
    }

    fn get_projector(pwd: PathBuf) -> Projector {
        return Projector {
            config: PathBuf::from(""),
            pwd,
            data: Data {
                projector: get_data(),
            },
        };
    }

    #[test]
    fn get_value() {
        let proj = get_projector("/foo".into());
        assert_eq!(Some(&String::from("bar2")), proj.get_value("foo"));
        assert_eq!(Some(&String::from("is great")), proj.get_value("fem"));
    }
    #[test]
    fn set_value() {
        let mut proj = get_projector(PathBuf::from("/foo/bar"));
        proj.set_value(String::from("foo"), String::from("bar4"));
        assert_eq!(proj.get_value("foo"), Some(&String::from("bar4")));
        proj.set_value(String::from("baz"), String::from("other"));
        assert_eq!(proj.get_value("baz"), Some(&String::from("other")));
        proj.set_value(String::from("fem"), String::from("is better than great"));
        assert_eq!(
            proj.get_value("fem"),
            Some(&String::from("is better than great"))
        );
    }
    #[test]
    fn remove_value() {
        let mut proj = get_projector(PathBuf::from("/foo/bar"));
        proj.remove_value("foo");
        proj.remove_value("fem");
        assert_eq!(proj.get_value("foo"), Some(&String::from("bar2")));
        assert_eq!(proj.get_value("fem"), Some(&String::from("is great")));
    }
    #[test]
    fn get_value_all() {
        let proj = get_projector(PathBuf::from("/foo"));

        let got = proj.get_value_all();

        let foo = &String::from("foo");
        let bar2 = &String::from("bar2");
        let fem = &String::from("fem");
        let is_great = &String::from("is great");

        let want = hashmap! {
            foo => bar2,
            fem => is_great,
        };

        assert_eq!(got, want);
    }
}
