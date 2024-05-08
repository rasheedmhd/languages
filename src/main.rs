use reqwest;

fn main() {
    println!("A web scrapper to detect career pages");
    let site = "https://remoteintech.company/";
    let response = reqwest::blocking::get(site);
    let data = response.unwrap().text().unwrap();

    println!("{data}");
}
