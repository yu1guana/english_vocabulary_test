// Copyright (c) 2022 Yuichi Ishida

use crate::card::CardList;
use crate::exam_maker::ExamMaker;
use anyhow::Result;
use clap::{Parser, ValueHint};
use std::path::PathBuf;

impl Cli {
    pub fn run() -> Result<()> {
        let args = Cli::parse();
        let card_list = CardList::read_card_list_from_file(&args.card_file)?;
        let exam_maker = ExamMaker::try_new(&args.card_file)?;
        exam_maker.make_exam_tex_file(&card_list, args.num_problem, &mut rand::thread_rng())?;
        Ok(())
    }
}

#[derive(Parser)]
#[clap(
    name = env!("CARGO_PKG_NAME"),
    author = env!("CARGO_PKG_AUTHORS"),
    version = env!("CARGO_PKG_VERSION"),
    about = "英単語テスト作成"
)]
pub struct Cli {
    #[clap(value_hint(ValueHint::FilePath))]
    card_file: PathBuf,
    num_problem: usize,
}
