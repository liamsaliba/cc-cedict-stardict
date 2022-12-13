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
    /// Returns the (lowest) hsk level of this [`CedictEntry`].
    pub fn hsk(&self) -> Option<hsk::Hsk> {
        hsk::HSK.level(&self.simplified)
    }

    #[allow(dead_code)]
    pub fn is_surname(&self) -> bool {
        self.entries.iter().any(|e| e.contains("surname "))
    }

    /// Is the simplified chinese different to the traditional chinese for this [`CedictEntry`]?
    pub fn has_simplified(&self) -> bool {
        self.simplified != self.traditional
    }

    pub fn to_simplified_csv_line(&self) -> String {
        format!("{}\t{}", self.simplified, self)
    }

    pub fn to_traditional_csv_line(&self) -> String {
        format!("{}\t{}", self.traditional, self)
    }
}

impl FromStr for CedictEntry {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (traditional, rest) = s
            .split_once(' ')
            .context("should split into traditional and rest")?;
        let (simplified, rest) = rest.split_once(" [").context("should split simplified")?;
        let (pinyin, rest) = rest.split_once("] /").context("should split pinyin")?;
        let mut entries: Vec<String> = rest.split('/').map(|s| s.to_string()).collect();
        entries.pop();

        let pinyin = prettify_pinyin::prettify(pinyin.to_string());

        Ok(Self {
            simplified: simplified.to_string(),
            traditional: traditional.to_string(),
            pinyin,
            entries,
        })
    }
}

impl Display for CedictEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let traditional = if self.has_simplified() {
            format!(" {}", &self.traditional)
        } else {
            "".into()
        };

        let entries: String = self
            .entries
            .iter()
            .map(|e| "• ".to_owned() + e + "\n")
            .collect();

        let hsk = self
            .hsk()
            .map_or_else(|| "".to_string(), |hsk| format!(" HSK {}", hsk));

        write!(
            f,
            "{} 【{}{}】{}\n{}",
            &self.pinyin, &self.simplified, traditional, hsk, entries,
        )
    }
}

// penelope -i cedict.csv -j csv --csv-fs "\t" --csv-ls "\n\n" -f cn -t en -p stardict -o cedict.zip --merge-definitions --merge-separator "\n" -d --title "CC-CEDICT 汉英词典"
// unzip cedict.zip
// sdcv --utf8-input --utf8-output --data-dir . -c -e 我
// sdcv --utf8-input --utf8-output --data-dir . -c -e -n --json 跑步
