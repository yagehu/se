use std::path::{self, Path};

use config::{Config, ConfigError};
use serde::de::DeserializeOwned;

pub fn new<T: DeserializeOwned + Fragment>() -> Result<T, ConfigError> {
    let mut builder = Config::builder();

    for segments in T::paths() {
        builder = builder.add_source(config::File::new(
            &segments.join(path::MAIN_SEPARATOR_STR),
            config::FileFormat::Yaml,
        ));
    }

    builder
        .add_source(
            config::Environment::with_prefix("SE")
                .prefix_separator("__")
                .separator("__"),
        )
        .build()?
        .try_deserialize()
}

pub trait Fragment {
    fn paths() -> Vec<Vec<&'static str>>;
}
