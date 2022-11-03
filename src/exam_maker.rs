// Copyright (c) 2022 Yuichi Ishida

use crate::card::CardList;
use anyhow::{anyhow, Context, Result};
use rand::seq::SliceRandom;
use rand::Rng;
use std::env;
use std::fmt::Write as _;
use std::fs;
use std::fs::File;
use std::io;
use std::io::{BufWriter, Write};
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Clone, Debug)]
pub struct ExamMaker {
    work_dir: PathBuf,
    exam_tex_file_name: String,
}

impl ExamMaker {
    const MAKE_EXAM_FILE_NAME: &'static str = "/tmp/make_exam.sh";

    pub(crate) fn try_new(card_file: &Path) -> Result<Self> {
        if !card_file.is_file() {
            return Err(anyhow!("{} does not exist", card_file.display()));
        }
        let file_stem = card_file.file_stem().unwrap().to_str().unwrap();
        Ok(Self {
            work_dir: card_file
                .canonicalize()
                .unwrap()
                .parent()
                .unwrap()
                .to_path_buf(),
            exam_tex_file_name: format!("exam_of_{}.tex", file_stem),
        })
    }

    pub(crate) fn make_exam_tex_file<R: Rng + ?Sized>(
        &self,
        card_list: &CardList,
        num_problem: usize,
        rng: &mut R,
    ) -> Result<()> {
        let pwd_dir = env::current_dir()?;
        env::set_current_dir(&self.work_dir)?;
        let mut buf_writer = BufWriter::new(File::create(&self.exam_tex_file_name)?);
        write!(
            &mut buf_writer,
            "{}",
            self.make_exam_tex_string(card_list, num_problem, rng)
        )?;
        env::set_current_dir(pwd_dir)?;
        Ok(())
    }

    pub(crate) fn make_exam_pdf_file(&self) -> Result<()> {
        let pwd_dir = env::current_dir()?;
        env::set_current_dir(&self.work_dir)?;
        self.make_latexmk_script()?;
        let output = Command::new(Self::MAKE_EXAM_FILE_NAME)
            .output()
            .context("failed to execute latexmk")?;
        io::stdout().write_all(&output.stdout).unwrap();
        io::stderr().write_all(&output.stderr).unwrap();
        fs::remove_file(Self::MAKE_EXAM_FILE_NAME)?;
        env::set_current_dir(pwd_dir)?;
        Ok(())
    }

    fn make_exam_tex_string<R: Rng + ?Sized>(
        &self,
        card_list: &CardList,
        num_problem: usize,
        rng: &mut R,
    ) -> String {
        let mut tex_string = String::new();
        writeln!(
            tex_string,
            "{}",
            r#"\documentclass[a4paper,11pt]{jsarticle}"#
        )
        .unwrap();
        writeln!(
            tex_string,
            "{}",
            r#"\usepackage[top=4truecm,bottom=2truecm,left=2truecm,right=2truecm]{geometry}"#
        )
        .unwrap();
        writeln!(tex_string, "{}", r#"\pagestyle{empty}"#).unwrap();
        writeln!(
            tex_string,
            "{}",
            r#"\renewcommand{\labelenumi}{(\arabic{enumi})}"#
        )
        .unwrap();
        writeln!(tex_string, "{}", r#"\begin{document}"#).unwrap();
        writeln!(tex_string, "{}", r#"\begin{enumerate}"#).unwrap();
        writeln!(tex_string, "{}", r#"  \setlength{\itemsep}{2truecm}"#).unwrap();
        let mut random_card_list_according_to_priority =
            card_list.pick_up_cards_randomly_according_to_priority(num_problem, rng);
        random_card_list_according_to_priority.shuffle(rng);
        for (i, card) in random_card_list_according_to_priority
            .into_iter()
            .enumerate()
        {
            writeln!(tex_string, "  {}", r#"\item"#).unwrap();
            writeln!(tex_string, "    {}", card.to_tex_string()).unwrap();
            if (i + 1) % 10 == 0 {
                writeln!(tex_string, r#"  \clearpage"#).unwrap();
            }
        }
        writeln!(tex_string, "{}", r#"\end{enumerate}"#).unwrap();
        writeln!(tex_string, "{}", r#"\end{document}"#).unwrap();
        tex_string
    }

    fn make_latexmk_script(&self) -> Result<()> {
        let make_exam_file = File::create(Self::MAKE_EXAM_FILE_NAME)?;
        let mut buf_writer = BufWriter::new(&make_exam_file);
        writeln!(buf_writer, "#!/bin/bash")?;
        writeln!(buf_writer, "latexmk {}", &self.exam_tex_file_name)?;
        writeln!(buf_writer, "latexmk -c {}", &self.exam_tex_file_name)?;
        let mut perms = make_exam_file.metadata()?.permissions();
        perms.set_mode(0o744);
        make_exam_file.set_permissions(perms)?;
        Ok(())
    }
}
