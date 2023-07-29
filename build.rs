use std::{
    env,
    fs::File,
    io::{self, BufRead, BufWriter, Write},
    path::Path,
    sync::{Arc, Mutex},
};

use rayon::prelude::{IntoParallelIterator, ParallelIterator};

const DICTIONARIES: [(&str, &str); 6] = [
    (
        "PASSWORDS",
        concat!(env!("CARGO_MANIFEST_DIR"), "/data/passwords.txt"),
    ),
    (
        "ENGLISH_WIKI",
        concat!(env!("CARGO_MANIFEST_DIR"), "/data/english_wiki.txt"),
    ),
    (
        "FEMALE_NAMES",
        concat!(env!("CARGO_MANIFEST_DIR"), "/data/female_names.txt"),
    ),
    (
        "MALE_NAMES",
        concat!(env!("CARGO_MANIFEST_DIR"), "/data/male_names.txt"),
    ),
    (
        "SURNAMES",
        concat!(env!("CARGO_MANIFEST_DIR"), "/data/surnames.txt"),
    ),
    (
        "US_TV_AND_FILM",
        concat!(env!("CARGO_MANIFEST_DIR"), "/data/us_tv_and_film.txt"),
    ),
];

fn main() -> io::Result<()> {
    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("codegen_dictionaries.rs");
    let file = Arc::new(Mutex::new(BufWriter::new(File::create(path)?)));

    DICTIONARIES.into_par_iter().try_for_each(|(name, path)| {
        let mut file = file.lock().map_err(|_| io::ErrorKind::Other)?;
        add_dictionary(&mut file, name, path)
    })?;

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=data/");

    Ok(())
}

fn add_dictionary(
    file: &mut BufWriter<File>,
    name: &str,
    filename: impl AsRef<Path>,
) -> io::Result<()> {
    let dict = read_dictionary(filename)?;
    write!(
        file,
        "static {}: phf::Map<&'static str, usize> = {}",
        name,
        dict.build(),
    )?;

    writeln!(file, ";")?;

    Ok(())
}

fn read_dictionary(filename: impl AsRef<Path>) -> io::Result<phf_codegen::Map<String>> {
    let mut dict = phf_codegen::Map::new();
    for (i, v) in read_lines(filename)?.enumerate() {
        dict.entry(v?, &format!("{}", i + 1));
    }

    Ok(dict)
}

fn read_lines(filename: impl AsRef<Path>) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::with_capacity(1024 * 1024, file).lines())
}
