use clap::Parser;
use regex::Regex;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path on the vimwiki root dir
    #[arg(short, long)]
    path: String,
}

fn main() {
    let args = Args::parse();

    // let files: Vec<String> = std::fs::read_dir(args.path)
    //     .unwrap()
    //     .map(|f| f.unwrap().path().display().to_string())
    //     .collect();
    // println!("{:#?}", files);

    let files: Vec<_> = std::fs::read_dir(&args.path)
        .unwrap()
        .map(|f| f.unwrap().file_name())
        .filter(|f| f.to_str().unwrap().ends_with(".md"))
        .collect();
    // println!("{:#?}", files);

    // let files: Vec<_> = std::fs::read_dir(args.path)
    //     .unwrap()
    //     .map(|f| {
    //         let file_name = f.unwrap().file_name();
    //         file_name.as_ref().and_then(OsStr::to_str).map(String::from)
    //     })
    //     .filter_map(|x| x)
    //     .collect();
    // println!("{:#?}", files);

    // map[filename] -> []string{"filename1", "filename2"}
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
                // println!("{:#?}{:#?}", file_name, line);
                re.captures(line)
                    .unwrap()
                    .get(1)
                    .unwrap()
                    .as_str()
                    .to_string()
                // let link = line
                //     .trim_start_matches("[[")
                //     .trim_end_matches("]]")
                //     .split('|')
                //     .next()
                //     .unwrap();
                // link.to_string()
            })
            .collect();
        // println!("{:#?}", links);
        map.insert(file_name, links);
    }

    map.into_iter().for_each(|i| println!("{:#?}", i));

    todo!("draw graph using javascript");
}
