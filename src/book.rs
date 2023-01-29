// Copyright (c) 2023 Yuichi Ishida
//
// Released under the MIT license.
// see https://opensource.org/licenses/mit-license.php

//! 単語カードリストとそのデータファイル。

use crate::card::{Card, CardList};
use anyhow::Result;
use getset::Getters;
use rand::seq::SliceRandom;
use rand::Rng;
use std::path::{Path, PathBuf};

/// 単語カードリストとそのデータファイル。
#[derive(Clone, Default, Debug, Getters)]
pub(crate) struct Book {
    #[getset(get = "pub(crate)")]
    card_file: PathBuf,
    card_list: CardList,
}

impl Book {
    pub(crate) fn try_new(file: &Path) -> Result<Self> {
        Ok(Self {
            card_file: file.to_path_buf(),
            card_list: CardList::read_card_list_from_file(file)?,
        })
    }

    /// 単語カードリストから重要度に従って単語カードを抽出する。
    ///
    /// 重要度の最低値が1となるようにオフセットを足したあと、重要度の比が確率の比となるように抽出する。
    pub(crate) fn pick_up_card_list_randomly_according_to_priority<R: Rng + ?Sized>(
        &self,
        max_num_problem: usize,
        rng: &mut R,
    ) -> Vec<Card> {
        let priority_offset = 1 - self
            .card_list
            .card()
            .iter()
            .map(|c| c.priority())
            .min()
            .unwrap();
        let mut priority_list = self
            .card_list
            .card()
            .iter()
            .map(|c| c.priority() + priority_offset)
            .collect::<Vec<_>>();
        let mut sum_of_priority = priority_list.iter().sum();
        let mut ret = Vec::with_capacity(max_num_problem);
        for _ in 0..max_num_problem {
            if sum_of_priority <= 0 {
                break;
            }
            let mut r = rng.gen_range(0..sum_of_priority);
            for (priority, card) in priority_list.iter_mut().zip(self.card_list.card().iter()) {
                r -= *priority;
                if r < 0 {
                    ret.push(card.clone());
                    sum_of_priority -= *priority;
                    *priority = 0;
                    break;
                }
            }
        }
        ret.shuffle(rng);
        ret
    }
}
