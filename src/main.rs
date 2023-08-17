#![allow(dead_code, unused_variables)]

use std::collections::BTreeSet;
use std::include_str;

type DictType = BTreeSet<String>;
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    word: Option<String>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,
}

fn main() {
    let cli = Cli::parse();
    let word_string = cli.word.unwrap_or(String::from("aylwari"));
    let word = word_string.as_bytes();

    let dict: DictType = BTreeSet::from(
        include_str!("dict2.txt")
            .trim()
            .split("\n")
            .map(String::from)
            .collect::<BTreeSet<String>>(),
    );

    let words = all_permutations(&word, &dict, true);
}

fn power_set(set: Vec<u8>) -> Vec<Vec<u8>> {
    let items = set.clone();
    let mut sets = Vec::with_capacity((2 as usize).pow(set.len() as u32));
    for item in items.iter() {
        let mut other = sets
            .clone()
            .iter_mut()
            .map(|s: &mut Vec<u8>| {
                let mut r = s.clone();
                r.push(item.clone());
                r
            })
            .collect::<Vec<Vec<u8>>>();

        sets.append(&mut other);
        sets.push(vec![item.clone()]);
    }
    sets.push(vec![]);
    return sets;
}

fn all_permutations(word: &[u8], dict: &DictType, print_progress: bool) -> Vec<String> {
    let mut acc: Vec<String> = Vec::new();
    let indices = (0..word.len() as u8).collect::<Vec<u8>>();
    let mut indices_power_set: Vec<Vec<u8>> = power_set(indices);
    for idx_set in indices_power_set.iter_mut() {
        if idx_set.len() == 0 {
            continue;
        }
        generate_permut(word, dict, idx_set.len(), idx_set, &mut acc, print_progress);
    }
    return acc;
}

fn swap(arr: &mut Vec<u8>, src: usize, dst: usize) {
    let tmp = arr[dst];
    arr[dst] = arr[src];
    arr[src] = tmp;
}

fn generate_permut(
    word: &[u8],
    dict: &DictType,
    k: usize,
    arr: &mut Vec<u8>,
    acc: &mut Vec<String>,
    print_progress: bool,
) {
    if k == 1 {
        let mut permutation = String::with_capacity(word.len());
        arr.iter()
            .for_each(|i| permutation.push(word[*i as usize] as char));

        if dict.contains(&permutation) && !acc.contains(&permutation) {
            if print_progress {
                println!("{}", permutation);
            }
            acc.push(permutation);
        }
        return;
    } else {
        generate_permut(&word, dict, k - 1, arr, acc, print_progress);
        for i in 0..(k - 1) {
            if k % 2 == 0 {
                swap(arr, i, k - 1);
            } else {
                swap(arr, 0, k - 1);
            }
            generate_permut(&word, dict, k - 1, arr, acc, print_progress);
        }
    }
}
