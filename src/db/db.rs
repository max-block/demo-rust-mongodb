use mongodb::{error::Error, error::ErrorKind, sync::Client};
use url::{ParseError, Url};

use super::repositories::UserRepository;

pub struct DB {
    pub user: UserRepository,
}

impl DB {
    pub fn new(database_url: String) -> Result<DB, Error> {
        let client = Client::with_uri_str(&database_url)?;
        let database_name = parse_database_name_from_url(&database_url).unwrap();
        let user = UserRepository::new(client.database(&database_name).collection("user"));

        Ok(DB { user })
    }
}

fn parse_database_name_from_url(database_url: &str) -> Result<String, ParseError> {
    let parsed = Url::parse(database_url)?;
    let database_name = parsed.path()[1..].to_string();
    if database_name.is_empty() {
        Err(ParseError::RelativeUrlWithoutBase)
    } else {
        Ok(database_name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_database_name_from_url() {
        let res = parse_database_name_from_url("mongodb://localhost/test");
        assert_eq!(res.unwrap(), "test");

        let res = parse_database_name_from_url("mongodb/localhost/test");
        assert!(res.is_err());

        let res = parse_database_name_from_url("mongodb://localhost/");
        assert!(res.is_err());
    }
}
