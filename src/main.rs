// Working with subcommands is simple. There are a few key points to remember when working with
// subcommands in clap. First, Subcommands are really just Apps. This means they can have their own
// settings, version, authors, args, and even their own subcommands. The next thing to remember is
// that subcommands are set up in a tree like hierarchy.
//
// An ASCII art depiction may help explain this better. Using a fictional version of git as the demo
// subject. Imagine the following are all subcommands of git (note, the author is aware these aren't
// actually all subcommands in the real git interface, but it makes explanation easier)
//
//            Top Level App (git)                         TOP
//                           |
//    -----------------------------------------
//   /             |                \          \
// clone          push              add       commit      LEVEL 1
//   |           /    \            /    \       |
//  url      origin   remote    ref    name   message     LEVEL 2
//           /                  /\
//        path            remote  local                   LEVEL 3
//
// Given the above fictional subcommand hierarchy, valid runtime uses would be (not an all inclusive
// list):
//
// $ git clone url
// $ git push origin path
// $ git add ref local
// $ git commit message
//
// Notice only one command per "level" may be used. You could not, for example, do:
//
// $ git clone url push origin path
//
// It's also important to know that subcommands each have their own set of matches and may have args
// with the same name as other subcommands in a different part of the tree hierarchy (i.e. the arg
// names aren't in a flat namespace).
//
// In order to use subcommands in clap, you only need to know which subcommand you're at in your
// tree, and which args are defined on that subcommand.
//
// Let's make a quick program to illustrate. We'll be using the same example as above but for
// brevity sake we won't implement all of the subcommands, only a few.

use clap::{App, AppSettings, Arg};
use hex;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::Write;
use std::fs;
use std::path::Path;

fn main() {
    let matches = App::new("rgit")
        .about("Implement git in rust")
        .version("1.0")
        .author("William Mbotta")
        .subcommand(
            App::new("init")
                .about("init repos")
                .license("MIT OR Apache-2.0"),
        )
        .subcommand(
            App::new("write-tree")
                .about("list files")
                .license("MIT OR Apache-2.0"),
        )
        .subcommand(
            App::new("hash-object")
                .about("hash file")
                .arg(Arg::new("file").about("file to hash").required(true)),
        )
        .subcommand(
            App::new("cat-file")
                .about("print object knowing it hash")
                .arg(Arg::new("hash").about("hash to print").required(true)),
        )
        .subcommand(
            App::new("clone")
                .about("clones repos")
                .license("MIT OR Apache-2.0")
                .arg(Arg::new("repo").about("The repo to clone").required(true)),
        )
        .subcommand(
            App::new("push")
                .about("pushes things")
                .setting(AppSettings::SubcommandRequiredElseHelp)
                .subcommand(
                    App::new("remote") // Subcommands can have their own subcommands,
                        // which in turn have their own subcommands
                        .about("pushes remote things")
                        .arg(
                            Arg::new("repo")
                                .required(true)
                                .about("The remote repo to push things to"),
                        ),
                )
                .subcommand(App::new("local").about("pushes local things")),
        )
        .subcommand(
            App::new("add")
                .about("adds things")
                .author("Someone Else") // Subcommands can list different authors
                .version("v2.0 (I'm versioned differently") // or different version from their parents
                .setting(AppSettings::ArgRequiredElseHelp) // They can even have different settings
                .arg(
                    Arg::new("stuff")
                        .long("stuff")
                        .about("Stuff to add")
                        .takes_value(true)
                        .multiple_values(true),
                ),
        )
        .get_matches();

    // The most common way to handle subcommands is via a combined approach using
    // `ArgMatches::subcommand` which returns a tuple of both the name and matches
    match matches.subcommand() {
        Some(("clone", clone_matches)) => {
            // Now we have a reference to clone's matches
            println!("Cloning {}", clone_matches.value_of("repo").unwrap());
        }
        Some(("push", push_matches)) => {
            // Now we have a reference to push's matches
            match push_matches.subcommand() {
                Some(("remote", remote_matches)) => {
                    // Now we have a reference to remote's matches
                    println!("Pushing to {}", remote_matches.value_of("repo").unwrap());
                }
                Some(("local", _)) => {
                    println!("'git push local' was used");
                }
                _ => unreachable!(),
            }
        }
        Some(("add", add_matches)) => {
            // Now we have a reference to add's matches
            println!(
                "Adding {}",
                add_matches
                    .values_of("stuff")
                    .unwrap()
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        }
        Some(("init", _)) => {
            // Now we have a reference to add's matches
            println!("init a repo");
            data();
        }
        Some(("hash-object", hash_matches)) => {
            // Now we have a reference to clone's matches
            let file = hash_matches.value_of("file").unwrap();
            println!("Hashing {}", file);
            let mut data = fs::read(file);
            hash_object(data.unwrap(), None);
        }
        Some(("cat-file", hash_matches)) => {
            // Now we have a reference to clone's matches
            let hash = hash_matches.value_of("hash").unwrap();
            println!("display file {}", hash);
            cat_file(hash);
        }
        Some(("write-tree", hash_matches)) => {
            write_tree(Path::new("src/"));
        }

        None => println!("No subcommand was used"), // If no subcommand was used it'll match the tuple ("", None)
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachabe!()
    }

    // Continued program logic goes here...
}

static RGIT_DIR: &str = ".rgit";
static RGIT_DIR_OBJECT: &str = ".rgit/objects";

fn data() -> std::io::Result<()> {
    fs::create_dir(RGIT_DIR)
}
use sha1::{Digest, Sha1};
use std::error::Error;

fn hash_object(mut data:Vec<u8> , type_object: Option<&str>) -> Result<String, Box<dyn Error>> {
    // fs::create_dir(RGIT_DIR_OBJECT)?;
    let begin:Vec<u8> = [type_object.map(str::as_bytes).unwrap_or(b"blob"), b"\x00"].concat();
    data.splice(0..0, begin);
    let mut hasher = Sha1::new();
    hasher.update(&data);
    println!("{:?}", data);
    let hash = hex::encode(hasher.finalize());
    let path = format!("{}/objects/{}", RGIT_DIR, hash);
    let mut file = File::create(path)?;
    file.write(&data)?;
    Ok(hash)
}

fn cat_file(hash: &str) {
    println!(
        "print file {}",
        String::from_utf8_lossy(&get_object(hash, None).expect("string"))
    );
}

fn get_object(oid: &str, expected: Option<&str>) -> io::Result<Vec<u8>> {
    let file_path = format!("{}/objects/{}", RGIT_DIR, oid);
    let data = fs::read(file_path)?;
    let mut split_iter = io::Cursor::new(&data).split(b'\x00').map(|l| l.unwrap());
    let type_object = split_iter.next().unwrap();
    if String::from_utf8(type_object.clone()).expect("") != expected.unwrap_or("blob") {
        panic!("Expected {}, got {:?}", expected.unwrap(), type_object);
    }
    Ok(data)
}

fn write_tree(dir: &Path) -> Result<String, Box<dyn Error>> {
    let mut entries = vec![];
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            let path_to_string = path.to_str().unwrap();
            let _type;
            let oid;

            if path_to_string.contains(".ugit") {
                continue;
            }
            if path.is_dir() {
                //dbg!(&path);
                _type = "tree";
                oid = write_tree(&path)?;
            } else {
                //dbg!("file", &path);
                _type = "blob";
                oid = hash_object(fs::read(&path)?, None)?;

            }
            let filename = path.into_os_string().into_string().unwrap();
            entries.push((filename, oid, _type))
        }
    }

    let mut tree = String::new();
    for (filename, oid, _type) in entries.iter() {
        tree.push_str(&format!("{} {} {}\n", _type, oid, filename));
    }

    hash_object(tree.as_bytes().to_vec(), Some("tree"))
}