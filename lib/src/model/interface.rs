use crate::db::INode;
use crate::serde::{Error, Result};
use serde::de::DeserializeOwned;
use std::path::PathBuf;
use std::rc::Rc;

pub trait Model: INode
where
    Self: 'static + DeserializeOwned,
{
    fn load(path: PathBuf) -> Result<Rc<dyn INode>> {
        match std::fs::read_to_string(path.to_owned()) {
            Ok(content) => Ok(Rc::new(toml::from_str::<Self>(&content)?)),
            Err(error) => Err(Error::InternalError(format!(
                "could not read {:?}: {error}",
                path.to_owned()
            ))),
        }
    }
}
