use reqwest;
use scraper::{Html, Selector};

fn main() {
    println!("A web scrapper to detect career pages");
    let site = "https://remoteintech.company/";
    let response = reqwest::blocking::get(site);
    let data = response.unwrap().text().unwrap();
    //<td class="company-website">
    // <a href="https://www.15five.com" target="_blank" rel="noopener noreferrer">15five.com</a>
    // </td>
    // let document = Html::parse_document(data);

    let fragment = Html::parse_fragment(&data);
    let selector = Selector::parse("td").unwrap();

    for element in fragment.select(&selector) {
        // assert_eq!("td", element.value().name());
        println!("td value {:?}", element.value().name());
    }

    // println!("{:?}", document);
}
