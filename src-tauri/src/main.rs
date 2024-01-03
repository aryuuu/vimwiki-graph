// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use regex::Regex;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_graph])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn get_graph() -> String {
    // TODO: refactor this hardcoded path
    let files: Vec<_> = std::fs::read_dir("/home/fatt/.vimwiki")
        .unwrap()
        .map(|f| f.unwrap().file_name())
        .filter(|f| f.to_str().unwrap().ends_with(".md"))
        // TODO: remove the .md extension from the filename
        .map(|f| f.to_str().unwrap().to_string().strip_suffix(".md").unwrap().to_string())
        .collect();

    let mut map = std::collections::HashMap::<String, Vec<String>>::new();
    let mut list = vec![];
    let re = Regex::new(r"\[\[([^\]|]+)").unwrap();

    for file in files {
        let file_name = file.clone();
        let file_path = format!("{}/{}.md", "/home/fatt/.vimwiki", file_name);
        println!("file_path: {}", file_path);
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

        links.iter().for_each(|i| {
            list.push(vec![file_name.clone(), i.clone()]);
        });
        map.insert(file_name, links);
    }

    serde_json::to_string(&list).unwrap()
}
