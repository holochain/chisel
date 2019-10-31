use heck::SnekCase;
use itertools::Itertools;
use regex::Regex;
use std::{
    collections::hash_map::{Entry, HashMap},
    fs::File,
    io::{self, Write},
    path::{Path, PathBuf},
};

#[derive(Debug, StructOpt)]
pub struct DemuxCmd {
    #[structopt(short, long)]
    dir: PathBuf,

    #[structopt(short)]
    verbose: bool,
}

impl DemuxCmd {
    pub fn run<R: io::BufRead>(&self, i: R) -> Result<(), String> {
        if !self.dir.is_dir() {
            return Err(format!(
                "{:?} is not a directory. Check that it exists.",
                self.dir
            ));
        }
        let re_intro = Regex::new(r"^(.){3} \{\{\{(.*?)\}\}\}$").unwrap();
        let mut files: HashMap<String, File> = HashMap::new();
        let mut active_conductor: Option<(char, String)> = None;
        for line in i.lines() {
            let line = line.map_err(|e| e.to_string())?;
            if let Some(caps) = re_intro.captures(&line) {
                if let (Some(prefix), Some(name)) = (caps.get(1), caps.get(2)) {
                    let prefix = prefix.as_str().chars().next().unwrap();
                    let name = name.as_str().to_owned();
                    active_conductor = Some((prefix, name));
                }
            } else {
                if let Some((ref prefix, ref name)) = active_conductor {
                    if line.starts_with(prefix.clone()) {
                        match files.entry(name.clone()) {
                            Entry::Occupied(entry) => writeln!(
                                entry.get(),
                                "{}",
                                line.chars().skip(2).collect::<String>()
                            )
                            .map_err(|e| e.to_string())?,
                            Entry::Vacant(entry) => {
                                let path = log_filename(&self.dir, name);
                                let file = File::create(path).unwrap();
                                entry.insert(file);
                            }
                        }
                    }
                }
            }
        }
        for file in files.values_mut() {
            file.flush().map_err(|e| e.to_string())?;
        }
        if self.verbose {
            println!(
                "\nchisel: extracted {} conductor logs from input:",
                files.len()
            );
            for name in files
                .keys()
                .map(|name| log_filename(&self.dir, name))
                .sorted()
            {
                println!("{}", name.to_string_lossy())
            }
        }
        Ok(())
    }
}

fn log_filename(dir: &Path, name: &String) -> PathBuf {
    let mut path = PathBuf::new();
    let filename = format!("{}.log", name.to_snek_case());
    path.push(dir);
    path.push(Path::new(&filename));
    path
}
