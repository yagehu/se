use serde::Deserialize;

#[derive(Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct Config {
    pub database: Database,
}

impl cfg::Fragment for Config {
    fn paths() -> Vec<Vec<&'static str>> {
        let mut v = vec![vec!["config", "piston", "server", "base", "main"]];

        v.extend(Database::paths());
        v
    }
}

#[derive(Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct Database {
    pub host:     String,
    pub port:     u16,
    pub name:     String,
    pub username: String,
    pub password: String,
}

impl cfg::Fragment for Database {
    fn paths() -> Vec<Vec<&'static str>> {
        vec![vec!["config", "piston", "server", "base", "database"]]
    }
}
