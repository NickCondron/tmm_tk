use std::process::{Child, Command};
#[allow(unused_variables)]
use std::{env, ffi::OsStr, fs, path::PathBuf, process};

use clap::Parser;

#[derive(Parser)]
#[command(name = "TrainingMode-More Tool Kit")]
#[command(version = "0.1")]
#[command(about = "Build tool for TrainingMode-More", long_about = None)]
struct Cli {
    #[arg(short = 'l', long = "link", value_name = "FILE", help = "Link file")]
    link: PathBuf,

    #[arg(
        short = 't',
        long = "table",
        value_name = "FILE",
        help = "Function table file"
    )]
    function_table: PathBuf,

    #[arg(short = 'd', long = "dat", value_name = "DAT", help = ".dat file")]
    dat: PathBuf,

    #[arg(
        short = 's',
        long = "symbol",
        value_name = "SYMBOL",
        help = "Symbol name",
        default_value = "evFunction"
    )]
    symbol: String,

    #[arg(
        short = 'b',
        long = "build",
        value_name = "DIR",
        help = "Build directory",
        default_value = "build"
    )]
    build_dir: PathBuf,

    #[arg(required(true), help = "File(s) to compile")]
    files: Vec<PathBuf>,

    #[arg(last = true, help = "Flags passed to gcc")]
    gcc_flags: Vec<String>,
}

#[derive(Clone, Debug)]
struct Links {
    addresses: Vec<u32>,
    names: Vec<String>,
}

fn parse_link_file(path: PathBuf) -> Option<Links> {
    match path.extension().map(OsStr::to_string_lossy) {
        None => {
            eprintln!("Warning: link file missing extension .link");
        }
        Some(s) if s != "link" => {
            eprintln!("Warning: unexepected extension for link file: {}", s);
        }
        _ => {}
    }
    match std::fs::read_to_string(&path) {
        Err(e) => {
            eprintln!("Error opening file {:?}: {}", path, e);
            return None;
        }
        Ok(text) => {
            let mut links = Links {
                addresses: Vec::new(),
                names: Vec::new(),
            };
            for (i, line) in text.lines().enumerate() {
                let Some((address_str, name)) = line.split_once(":") else {
                    eprintln!("Error: Failed to parse link file line {}: {}", i + 1, line);
                    return None;
                };
                let Ok(address) = u32::from_str_radix(address_str, 16) else {
                    eprintln!(
                        "Error: Invalid address on link file line {}: {}",
                        i + 1,
                        address_str
                    );
                    return None;
                };
                if address_str.len() != 8 {
                    eprintln!(
                        "Warning: address on link file line {} is not 8 digits : {}",
                        i + 1,
                        address_str
                    );
                }
                links.addresses.push(address);
                links.names.push(name.to_string());
            }
            Some(links)
        }
    }
}

fn create_build_dir(dir: &PathBuf) {
    if dir.exists() {
        if !dir.is_dir() {
            eprintln!("Error: {:?} is not a directory", dir);
            process::exit(1);
        }
        // directory already exists
    }
    if let Err(e) = fs::create_dir_all(&dir) {
        eprintln!("Error creating directory {:?}: {}", dir, e);
        process::exit(1);
    }
    println!("Created build directory {:?}", dir);
}

fn get_gcc_path() -> PathBuf {
    if cfg!(target_os = "windows") {
        // TODO more robust solution
        PathBuf::from("C:/devkitpro/devkitPPC/bin/powerpc-eabi-gcc.exe")
    } else {
        // Linux
        match env::var("DEVKITPRO") {
            Ok(path) => PathBuf::from(path).join("devkitPPC/bin/powerpc-eabi-gcc"),
            Err(e) => {
                eprintln!("Failed to retrieve DEVKITPRO env variable: {}", e);
                process::exit(1);
            }
        }
    }
}

fn main() {
    let args = Cli::parse();

    println!("link: {:?}", args.link);
    println!("function_table: {:?}", args.function_table);
    println!("dat: {:?}", args.dat);
    println!("symbol: {:?}", args.symbol);
    println!("build_dir: {:?}", args.build_dir);
    println!("files: {:?}", args.files);
    println!("gcc flags: {:?}", args.gcc_flags);

    let Some(_links) = parse_link_file(args.link) else {
        process::exit(1);
    };

    let functions: Vec<String> = match std::fs::read_to_string(&args.function_table) {
        Ok(s) => s.lines().map(str::to_string).collect(),
        Err(e) => {
            eprintln!(
                "Could not read function table file {:?}: {}",
                args.function_table, e
            );
            process::exit(1);
        }
    };

    create_build_dir(&args.build_dir);
    let gcc_path = get_gcc_path();

    // 1. compile inputs to object files (*.o)
    let mut objs: Vec<PathBuf> = Vec::new();
    for file in args.files.iter() {
        if file.extension().map(OsStr::to_string_lossy) != Some("c".into()) {
            // MexTK allows .o inputs but I see no reason to for now
            eprintln!("Error: received nonsource input file: {:?}", file);
            process::exit(1);
        }
        let obj_file = args
            .build_dir
            .join(file.file_name().unwrap())
            .with_extension("o");

        Command::new(&gcc_path)
            .args(args.gcc_flags.iter())
            .args(["-DGEKKO", "-mogc", "-mcpu=750", "-meabi", "-mhard-float"])
            .arg("-c")
            .arg(file)
            .arg("-o")
            .arg(&obj_file)
            .output()
            .expect("failed to execute child process");
        objs.push(obj_file);
    }
}
