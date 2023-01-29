// Copyright (c) 2023 Yuichi Ishida
//
// Released under the MIT license.
// see https://opensource.org/licenses/mit-license.php

//! 試験問題の作成。

use crate::book::Book;
use crate::card::Card;
use anyhow::{Context, Result};
use rand::Rng;
use std::env;
use std::fmt::Write as _;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::path::PathBuf;

/// 単語カードリストから試験問題と解答を作成する。
///
/// 問題と解答はカードファイルがあるディレクトリに出力される。
#[derive(Clone, Debug)]
pub struct ExamMaker {
    book: Book,
    work_dir: PathBuf,
    exam_tex_file: PathBuf,
    answer_tex_file: PathBuf,
}

impl ExamMaker {
    pub(crate) fn new(book: Book) -> Self {
        let card_file = book.card_file();
        let work_dir = card_file
            .canonicalize()
            .unwrap()
            .parent()
            .unwrap()
            .to_path_buf();
        let exam_tex_file = work_dir.join(format!(
            "exam_of_{}.tex",
            card_file.file_stem().unwrap().to_str().unwrap()
        ));
        let answer_tex_file = work_dir.join(format!(
            "answer_of_{}.tex",
            card_file.file_stem().unwrap().to_str().unwrap()
        ));
        Self {
            book,
            work_dir,
            exam_tex_file,
            answer_tex_file,
        }
    }

    /// 試験問題と解答をtexファイルに出力。
    pub(crate) fn write_tex_file<R: Rng + ?Sized>(
        &self,
        num_problem: usize,
        rng: &mut R,
    ) -> Result<()> {
        let exam_card_list = self
            .book
            .pick_up_card_list_randomly_according_to_priority(num_problem, rng);
        write_tex_file(&self.work_dir, &self.exam_tex_file, &exam_card_list, true)?;
        write_tex_file(
            &self.work_dir,
            &self.answer_tex_file,
            &exam_card_list,
            false,
        )?;
        Ok(())
    }
}

/// 試験問題もしくは解答をtexファイルに出力。
///
/// # Arguments
/// - `flag_exam`: 試験問題の場合は`true`を指定。
fn write_tex_file(
    work_dir: &Path,
    tex_file: &Path,
    card_list: &[Card],
    flag_exam: bool,
) -> Result<()> {
    let pwd_dir = env::current_dir().context("failed to get the current directory")?;
    env::set_current_dir(work_dir).with_context(|| {
        format!(
            "failed to change the current directory to {}",
            work_dir.display()
        )
    })?;
    let mut buf_writer = BufWriter::new(
        File::create(tex_file)
            .with_context(|| format!("failed to create {}", tex_file.display()))?,
    );
    write!(&mut buf_writer, "{}", make_tex_string(card_list, flag_exam))
        .with_context(|| format!("failed to write latex strings to {}", tex_file.display()))?;
    env::set_current_dir(&pwd_dir).with_context(|| {
        format!(
            "failed to change the current directory to {}",
            pwd_dir.display()
        )
    })?;
    Ok(())
}

/// texファイルに出力するための文字列を作成。
fn make_tex_string(card_list: &[Card], flag_exam: bool) -> String {
    let mut tex_string = String::new();
    writeln!(tex_string, "\\documentclass[a4paper,11pt]{{jsarticle}}").unwrap();
    writeln!(
        tex_string,
        "\\usepackage[top=4truecm,bottom=2truecm,left=2truecm,right=2truecm]{{geometry}}"
    )
    .unwrap();
    writeln!(tex_string, "\\pagestyle{{empty}}").unwrap();
    writeln!(
        tex_string,
        "\\renewcommand{{\\labelenumi}}{{(\\arabic{{enumi}})}}"
    )
    .unwrap();
    writeln!(tex_string, "\\begin{{document}}").unwrap();
    writeln!(tex_string, "\\begin{{enumerate}}").unwrap();
    writeln!(tex_string, "  \\setlength{{\\itemsep}}{{2truecm}}").unwrap();
    for (i, card) in card_list.iter().enumerate() {
        writeln!(tex_string, "  \\item").unwrap();
        writeln!(
            tex_string,
            "    {}",
            if flag_exam {
                card.exam_tex_string()
            } else {
                card.answer_tex_string()
            }
        )
        .unwrap();
        if (i + 1) % 10 == 0 {
            writeln!(tex_string, "  \\clearpage").unwrap();
        }
    }
    writeln!(tex_string, "\\end{{enumerate}}").unwrap();
    writeln!(tex_string, "\\end{{document}}").unwrap();
    tex_string
}
