use clap::Parser;
use regex::Regex;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path on the vimwiki root dir
    #[arg(short, long)]
    path: String,
}

fn main() {
    let args = Args::parse();

    let files: Vec<_> = std::fs::read_dir(&args.path)
        .unwrap()
        .map(|f| f.unwrap().file_name())
        .filter(|f| f.to_str().unwrap().ends_with(".md"))
        .collect();

    let mut map = std::collections::HashMap::<String, Vec<String>>::new();
    let re = Regex::new(r"\[\[([^\]|]+)").unwrap();

    for file in files {
        let file_name = file.to_str().unwrap().to_string();
        let file_path = format!("{}/{}", &args.path, file_name);
        let content = std::fs::read_to_string(file_path).unwrap();
        let links: Vec<_> = content
            .lines()
            .filter(|line| line.contains("[["))
            .map(|line| {
                re.captures(line)
                    .unwrap()
                    .get(1)
                    .unwrap()
                    .as_str()
                    .to_string()
            })
            .collect();

        map.insert(file_name, links);
    }

    map.into_iter().for_each(|i| println!("{:#?}", i));

    todo!("draw graph using javascript");
}
