use std::{fmt::Display, str::FromStr};

use anyhow::{Context, Error};

use crate::hsk;

pub struct CedictEntry {
    simplified: String,
    traditional: String,
    pinyin: String,
    entries: Vec<String>,
}

impl CedictEntry {
    #[allow(dead_code)]
    pub fn hsk(&self) -> Option<hsk::Hsk> {
        hsk::HSK.level(&self.simplified)
    }

    #[allow(dead_code)]
    pub fn is_surname(&self) -> bool {
        self.entries.iter().any(|e| e.contains("surname "))
    }
}

impl FromStr for CedictEntry {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (simplified, rest) = s
            .split_once(' ')
            .context("should split into simplified and rest")?;
        let (traditional, rest) = rest.split_once(" [").context("should split traditional")?;
        let (pinyin, rest) = rest.split_once("] /").context("should split pinyin")?;
        let mut entries: Vec<String> = rest.split('/').map(|s| s.to_string()).collect();
        entries.pop();

        Ok(Self {
            simplified: simplified.to_string(),
            traditional: traditional.to_string(),
            pinyin: pinyin.to_string(),
            entries,
        })
    }
}

impl Display for CedictEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let traditional = if self.traditional == self.simplified {
            "".into()
        } else {
            format!(" {}", &self.traditional)
        };

        write!(
            f,
            "{}\t{} 【{}{}】 {}",
            &self.simplified,
            &self.pinyin,
            &self.simplified,
            traditional,
            &self.entries.join("; ")
        )
    }
}
