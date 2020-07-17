use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader};
use std::sync::mpsc;

use crate::logparse::LogMessage;
use crate::markov::Markov;
use crate::msgprocessor::ProcessedMessage;

pub fn markov_from_logs(log_files: &'static [&str]) -> HashMap<String, Markov> {
    let (tx, rx) = mpsc::channel();

    for path in log_files.iter() {
        let tx = tx.clone();

        std::thread::spawn(move || {
            let file = OpenOptions::new()
                .read(true)
                .write(false)
                .create(false)
                .open(path)
                .unwrap();

            let reader = BufReader::new(file);

            for line in reader.lines() {
                let line = line.unwrap();

                if let Some(msg) = LogMessage::from_konversation(&line) {
                    let processed = ProcessedMessage::from(msg);

                    if !processed.user.is_empty() && !processed.words.is_empty() {
                        tx.send(processed).unwrap();
                    }
                };
            }
        });
    }

    // not sure if there is a better way - we clone the tx into each thread but we are not sending anything from the main thread.
    std::mem::drop(tx);

    let mut markovs = HashMap::new();

    for msg in rx {
        let markov = markovs.entry(msg.user).or_insert_with(|| Markov::new());

        if msg.words.is_empty() {
            continue;
        }

        for i in 0..msg.words.len() {
            let default_string = String::new();
            let prev = if i == 0 {
                &default_string
            } else {
                &msg.words[i - 1]
            };
            let cur = &msg.words[i];

            markov.insert(cur.clone(), prev.clone());
        }

        markov.insert(String::new(), msg.words.last().unwrap().to_string());
    }

    markovs
}
