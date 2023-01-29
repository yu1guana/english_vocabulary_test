// Copyright (c) 2023 Yuichi Ishida
//
// Released under the MIT license.
// see https://opensource.org/licenses/mit-license.php

//! 単語カード。

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

/// 単語カード
#[derive(Clone, CopyGetters, Default, Debug, Deserialize, Serialize)]
pub(crate) struct Card {
    /// 重要度。
    /// この値が大きいほど試験作成の際に選ばれやすいようにする。
    #[getset(get_copy = "pub(crate)")]
    priority: i64,

    /// 単語帳におけるページ番号。
    page: u64,

    /// 単語帳における単語番号。
    id: u64,

    /// 問題文となる英語。
    english: String,

    /// 解答となる文章。
    sentence: Option<String>,

    /// 節かどうかを表すフラグ。
    phrase: Option<bool>,

    /// 解答となる名詞。
    noun: Option<Vec<String>>,

    /// 解答となる形容詞。
    adjective: Option<Vec<String>>,

    /// 解答となる動詞。
    verb: Option<Vec<String>>,

    /// 解答となる副詞。
    adverb: Option<Vec<String>>,

    /// 解答となる前置詞。
    preposition: Option<Vec<String>>,
}

impl CardList {
    /// カードリストをファイルから読み込む。
    pub(crate) fn read_card_list_from_file(file: &Path) -> Result<Self> {
        let file_contents = fs::read_to_string(file)
            .with_context(|| format!("failed to read {}", file.display()))?;
        toml::from_str(&file_contents)
            .with_context(|| format!("failed to parse {}", file.display()))
    }

    /// カードリストから空のカードを削除する。
    pub(crate) fn drop_empty_card(&mut self) {
        self.card.retain(|card| !card.is_empty());
    }
}

impl Card {
    /// カードから試験問題の文字列を作成。
    pub(crate) fn exam_tex_string(&self) -> String {
        let mut tex_string = String::new();
        write!(tex_string, "p.{}", self.page).unwrap();
        write!(tex_string, "~\\#{}", self.id).unwrap();
        if self.sentence.is_some() && !self.sentence.as_ref().unwrap().is_empty() {
            write!(
                tex_string,
                " {} {}",
                tag(false, "文章"),
                self.sentence.as_ref().unwrap()
            )
            .unwrap();
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

    /// カードから解答の文字列を作成。
    pub(crate) fn answer_tex_string(&self) -> String {
        let mut tex_string = String::new();
        write!(tex_string, "p.{}", self.page).unwrap();
        write!(tex_string, "~\\#{}", self.id).unwrap();
        write!(tex_string, " {}", self.english).unwrap();
        tex_string
    }

    /// 空のカードを判定
    pub(crate) fn is_empty(&self) -> bool {
        let mut is_empty = self
            .sentence
            .as_ref()
            .map_or(true, |sentence| sentence.is_empty());
        is_empty = is_empty
            && [
                &self.noun,
                &self.adjective,
                &self.verb,
                &self.adverb,
                &self.preposition,
            ]
            .into_iter()
            .all(|word| word.as_ref().map_or(true, |word| word.is_empty()));
        is_empty
    }
}

/// 文字列にカードの意味のリストを書き込む。
fn write_meaning_list(
    phrase: bool,
    meaning_list: &Option<Vec<String>>,
    name: &str,
    tex_string: &mut String,
) {
    if let Some(meaning_list) = meaning_list {
        if !meaning_list.is_empty() {
            write!(
                tex_string,
                "  {} {}",
                tag(phrase, name),
                itertools::join(meaning_list.iter(), "、")
            )
            .unwrap()
        }
    }
}

/// 単語ではなく節の場合は品詞の後に「節」を加える。
fn tag(phrase: bool, name: &str) -> String {
    format!("[{}{}]", name, if phrase { "節" } else { "" })
}
