fn main() {
    let args_len = std::env::args().count() - 1;
    let pattern = std::env::args().nth(1).expect("no pattern provided");
    let path = std::env::args().nth(args_len).expect("no path provided");

    if args_len == 2 {
        let contents =
            std::fs::read_to_string(path.clone()).expect("Should have been able to read the file");
        if pattern == "-c" {
            println!("{} {}", contents.len(), path);
        } else if pattern == "-l" {
            let mut count = 0;
            for _ in contents.lines() {
                count += 1;
            }
            println!("{} {}", count, path);
        } else if pattern == "-w" {
            let mut count = 0;
            count += contents.split_whitespace().count();
            println!("{} {}", count, path);
        } else if pattern == "-m" {
            let mut count = 0;
            count += contents.chars().count();
            println!("{} {}", count, path);
        } else {
            println!("Unsupported flag {}", pattern);
        }
    } else {
        let contents =
            std::fs::read_to_string(path.clone()).expect("Should have been able to read the file");
        println!(
            "{} {} {} {}",
            contents.lines().count(),
            contents.split_whitespace().count(),
            contents.len(),
            path
        );
    }
}
