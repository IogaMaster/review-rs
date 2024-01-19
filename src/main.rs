fn main() {
    println!("Hello, world!");
    let args: Vec<String> = env::args().collect();

    let mut pr_num: String = String::new();

    if args.len() > 1 {
        if args[1].contains("github.com/nixos/nixpkgs") == true {
            pr_num = args[1].split("/").last().unwrap().to_string();
        } else if args[1].to_string().parse::<i32>().is_ok() {
            pr_num = args[1].to_string();
        };
    } else {
        println!("You need to specify the pr number or url");
        exit(1);
    }

    let mut pr_url: String = format!("https://api.github.com/repos/nixos/nixpkgs/pulls/{pr_num}");
}
