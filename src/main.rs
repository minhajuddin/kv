use clap::{Arg, Command};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = get_matches();
    let data_dir = dirs::data_dir().expect("Could not find data directory");
    let store = sled::open(data_dir.join("kv.db"))?;

    match matches.subcommand() {
        Some(("set", submatches)) => {
            let key = submatches.value_of("key").expect("No key specified");
            let val: String = submatches
                .values_of("val")
                .expect("No value specified")
                .collect::<Vec<_>>()
                .join(" ");
            store.insert(key, val.as_bytes())?;
        }

        Some(("get", submatches)) => {
            let key = submatches.value_of("key").expect("No key specified");
            if let Some(val) = store.get(key)? {
                let s = String::from_utf8(val.to_vec())?;
                println!("{}", s);
            } else {
                eprintln!("key not found");
                std::process::exit(1);
            }
        }

        Some(("del", submatches)) => {
            let key = submatches.value_of("key").expect("No key specified");
            store.remove(key)?;
        }

        Some(("list", _submatches)) => {
            for row in store.iter() {
                let (key, val) = row.expect("Could not read row");
                println!(
                    "{}\t{}",
                    String::from_utf8(key.to_vec())?,
                    String::from_utf8(val.to_vec())?
                );
            }
        }
        _ => unreachable!(),
    }

    Ok(())
}

fn get_matches() -> clap::ArgMatches {
    Command::new("kv")
        .about("A simple key-value store")
        .subcommand(
            Command::new("set")
                .about("Sets a key/value pair")
                .arg(
                    Arg::new("key")
                        .help("key to set")
                        .value_name("KEY")
                        .required(true),
                )
                .arg(
                    Arg::new("val")
                        .help("value to set")
                        .value_name("VAL")
                        .multiple_values(true)
                        .required(true),
                ),
        )
        .subcommand(
            Command::new("get")
                .about("Gets a value for a given key")
                .arg(
                    Arg::new("key")
                        .help("key to fetch")
                        .value_name("KEY")
                        .required(true),
                ),
        )
        .subcommand(
            Command::new("del")
                .about("Deletes a value for a given key")
                .arg(
                    Arg::new("key")
                        .help("key to delete")
                        .value_name("KEY")
                        .required(true),
                ),
        )
        .subcommand(Command::new("list").about("List all keys and values separated by tabs"))
        .subcommand_required(true)
        .get_matches()
}
