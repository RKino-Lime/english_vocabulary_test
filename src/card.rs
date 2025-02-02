// Copyright (c) 2023 Yuichi Ishida
//
// Released under the MIT license.
// see https://opensource.org/licenses/mit-license.php

use anyhow::{Context, Result};
use getset::{CopyGetters, Getters};
use serde_derive::{Deserialize, Serialize};
use std::fmt::Write as _;
use std::fs;
use std::path::Path;

#[derive(Clone, Default, Debug, Deserialize, Getters, Serialize)]
#[getset(get = "pub(crate)")]
pub(crate) struct CardList {
    card: Vec<Card>,
}

#[derive(Clone, CopyGetters, Default, Debug, Deserialize, Serialize)]
pub(crate) struct Card {
    #[getset(get_copy = "pub(crate)")]
    priority: i64,
    page: u64,
    id: u64,
    english: String,
    sentence: Option<String>,
    phrase: Option<bool>,
    noun: Option<Vec<String>>,
    adjective: Option<Vec<String>>,
    verb: Option<Vec<String>>,
    adverb: Option<Vec<String>>,
    preposition: Option<Vec<String>>,
}

impl CardList {
    pub(crate) fn read_card_list_from_file(file: &Path) -> Result<Self> {
        let file_contents = fs::read_to_string(file)
            .with_context(|| format!("failed to read {}", file.display()))?;
        toml::from_str(&file_contents)
            .with_context(|| format!("failed to parse {}", file.display()))
    }
}

impl Card {
    pub(crate) fn exam_tex_string(&self) -> String {
        let mut tex_string = String::new();
        write!(tex_string, "p.{}", self.page).unwrap();
        write!(tex_string, "~\\#{}", self.id).unwrap();
        if let Some(sentence) = &self.sentence {
            write!(tex_string, " {} {}", tag(false, "文章"), sentence).unwrap();
        } else {
            let phrase = self.phrase.map_or_else(|| false, |phrase| phrase);
            write_meaning_list(phrase, &self.noun, "名詞", &mut tex_string);
            write_meaning_list(phrase, &self.adjective, "形容詞", &mut tex_string);
            write_meaning_list(phrase, &self.verb, "動詞", &mut tex_string);
            write_meaning_list(phrase, &self.adverb, "副詞", &mut tex_string);
            write_meaning_list(phrase, &self.preposition, "前置詞", &mut tex_string);
        }
        tex_string
    }

    pub(crate) fn answer_tex_string(&self) -> String {
        let mut tex_string = String::new();
        write!(tex_string, "p.{}", self.page).unwrap();
        write!(tex_string, "~\\#{}", self.id).unwrap();
        write!(tex_string, " {}", self.english).unwrap();
        tex_string
    }
}

fn write_meaning_list(
    phrase: bool,
    meaning_list: &Option<Vec<String>>,
    name: &str,
    tex_string: &mut String,
) {
    meaning_list.as_ref().map(|meaning| {
        write!(
            tex_string,
            "  {} {}",
            tag(phrase, name),
            itertools::join(meaning.iter(), "、")
        )
    });
}

fn tag(phrase: bool, name: &str) -> String {
    format!("[{}{}]", name, if phrase { "節" } else { "" })
}
