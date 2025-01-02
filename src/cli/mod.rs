use std::{collections::HashMap, fs, path::PathBuf};

use args::{Cli, Priority};
use clap::Parser;

use crate::{Error, Result};

mod args;

pub fn parse() -> Result<HashMap<String, String>> {
    args::Cli::parse().try_into()
}

impl TryInto<HashMap<String, String>> for Cli {
    type Error = Error;

    fn try_into(self) -> Result<HashMap<String, String>> {
        let mut map = HashMap::with_capacity(10);

        let token = match self.token.token {
            Some(token) => token,
            None => read_line(self.token.token_file.unwrap())?,
        };

        let user = match self.user.user {
            Some(user) => user,
            None => read_line(self.user.user_file.unwrap())?,
        };

        if let Some(title) = self.title {
            map.insert("title".into(), title);
        };

        if let Some(device) = self.device {
            map.insert("device".into(), device);
        }

        if let Some(p) = self.priority {
            // Retry and expire are required in case of Emergency
            if p == Priority::Emergency {
                map.insert("retry".into(), "30".into()); // Retry each 30s
                map.insert("expire".into(), "90".into()); // 3 retries = 90s
            };

            let priority = match p {
                Priority::Lowest => -2,
                Priority::Low => -1,
                Priority::Normal => 0,
                Priority::High => 1,
                Priority::Emergency => 2,
            };

            map.insert("priority".into(), priority.to_string());
        }

        map.insert("message".into(), self.content);
        map.insert("token".into(), token);
        map.insert("user".into(), user);

        Ok(map)
    }
}

fn read_line(path: PathBuf) -> Result<String> {
    if !path.exists() {
        return Err(Error::FileNotFound(path));
    }

    fs::read_to_string(&path)?
        .lines()
        .next()
        .map(|s| s.to_owned())
        .ok_or(Error::FileIsEmpty(path))
}
