use reqwest::blocking::get;
use scraper::{Html, Selector};
use std::error::Error;

/*
 Author Gaurav Sablok
 Email: codeprog@icloud.com
 Date: 2025-8-8
*/

pub fn webminer(id: &str) -> Result<String, Box<dyn Error>> {
    let idnumber = id.parse::<usize>().unwrap();
    let formatstring_download = format!("{}/{}", "https://bacdive.dsmz.de/strain", idnumber);
    let infobox = get(&formatstring_download).expect("string not found");
    //let hiddenlinks = get(&formatstring_download).expect("string not found");
    //let seqlinks = get(&formatstring_download).expect("string not found");
    //let ncbilinks = get(&formatstring_download).expect("string not found");
    let document = Html::parse_document(&infobox.text().expect("message not present"));
    let snpselect = Selector::parse(".infobox_key").expect("method failed");
    for element in document.select(&snpselect) {
        let vector_1 = element.text().collect::<Vec<_>>().join("-");
        println!("{}", vector_1);
    }
    Ok("The webmine results are as follows".to_string())
}
