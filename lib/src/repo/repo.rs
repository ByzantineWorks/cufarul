use super::{Error, RepositorySpec, Result};

const SUPPORTED_VERSION: u8 = 0;

#[derive(Debug)]
pub struct Repository {
    spec: RepositorySpec,
}

impl TryFrom<RepositorySpec> for Repository {
    type Error = Error;
    fn try_from(spec: RepositorySpec) -> Result<Self> {
        if spec.version() != SUPPORTED_VERSION {
            return Err(Error::UnsupportedVersion(spec.version()));
        }

        Ok(Repository { spec: spec })
    }
}

impl Repository {
    pub fn spec(&self) -> &RepositorySpec {
        &self.spec
    }
}
