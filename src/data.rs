fn get_session() -> String {
    dotenvy::dotenv().unwrap();
    std::env::var("AOC_SESSION").expect("environment variable AOC_SESSION should be set")
}

pub fn get_day_input(day: usize) -> String {
    let file_name = format!("data/{day:02}.in");

    if std::fs::exists(&file_name).unwrap() {
        println!("Getting day {day} input from cache");
        return std::fs::read_to_string(&file_name).unwrap();
    }

    println!("Fetching day {day} input from AOC");

    let session = get_session();

    let response = ureq::get(&format!("https://adventofcode.com/2024/day/{day}/input"))
        .set("Cookie", &format!("session={session}"))
        .call()
        .unwrap();

    if response.status() != 200 {
        panic!("Request to AOC failed: {response:?}");
    }

    let content = response.into_string().unwrap();
    std::fs::write(file_name, &content).unwrap();
    content
}

pub fn get_day_output(day: usize) -> (Option<String>, Option<String>) {
    let file_name = format!("data/{day:02}.out");

    if !std::fs::exists(&file_name).unwrap() {
        return (None, None);
    }

    let content = std::fs::read_to_string(&file_name).unwrap();
    let mut lines = content.lines().map(|l| l.trim().to_owned());

    (lines.next(), lines.next())
}

pub fn set_day_output(day: usize, output1: &str, output2: &str) {
    println!("Saving output as expected output");
    let file_name = format!("data/{day:02}.out");
    std::fs::write(file_name, format!("{output1}\n{output2}")).unwrap();
}
