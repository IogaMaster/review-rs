use std::{collections::HashMap, env, error::Error, process::exit, borrow::Borrow};

use octocrab::models::pulls::FileDiff;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    let mut pr_num: String = String::new();

    if args.len() > 1 {
        if args[1].contains("github.com") == true {
            pr_num = args[1].split("/").last().unwrap().to_string();
        } else if args[1].to_string().parse::<i32>().is_ok() {
            pr_num = args[1].to_string();
        };
    } else {
        println!("You need to specify the pr number or url");
        exit(1);
    }

    let mut changed_packages: Vec<String> = vec![];

    for diff in octocrab::instance()
        .pulls("nixos", "nixpkgs")
        .list_files(pr_num.parse::<u64>().unwrap())
        .await.unwrap().items
    {
        // Only get packages, not top-level or other changed files
        if diff.filename.contains("pkgs") && !diff.filename.contains("top-level") {
            // let mut package_name = diff.filename.rsplit("/").nth(1).unwrap().to_string();
            changed_packages.push(diff.filename);
        }
    };
    
    println!("{:#?}", changed_packages) 
}
