// Copyright (c) 2023 Yuichi Ishida
//
// Released under the MIT license.
// see https://opensource.org/licenses/mit-license.php

//! Command line interface.

use crate::book::Book;
use crate::exam_maker::ExamMaker;
use anyhow::Result;
use clap::{Parser, ValueHint};
use std::path::PathBuf;

#[derive(Parser)]
#[clap(author, version, about, after_help = concat!("Repository: ", env!("CARGO_PKG_REPOSITORY")))]
pub struct Cli {
    #[clap(value_hint(ValueHint::FilePath))]
    card_file: PathBuf,
    num_problem: usize,
}

impl Cli {
    pub fn run() -> Result<()> {
        let args = Cli::parse();
        let book = Book::try_new(&args.card_file)?;
        let exam_maker = ExamMaker::new(book);
        exam_maker.write_tex_file(args.num_problem, &mut rand::thread_rng())?;
        Ok(())
    }
}
