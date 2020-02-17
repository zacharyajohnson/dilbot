extern crate reqwest;
extern crate select;
extern crate chrono;
  
use reqwest::Response;
use select::document::Document;
use select::predicate::{Class};
use std::io;
use std::fs::File;
use chrono::{Date, Utc, NaiveDate, Duration, Datelike};

fn main() {
    println!("Hello, world!");
    get_data();
}

 fn get_data(){
    const base_url: &str = "https://dilbert.com/strip/";
    let mut comic_date : NaiveDate = NaiveDate::from_ymd(1995, 6, 1);
    
    while comic_date != NaiveDate::from_ymd(2019, 10, 15) {
        let mut comic_date_string = String::default();
        
        comic_date_string.push_str(&comic_date.year().to_string());
        comic_date_string.push_str("-");

        if comic_date.month() < 9 {
            comic_date_string.push_str("0");
        }

        comic_date_string.push_str(&comic_date.month().to_string());
        comic_date_string.push_str("-");
        if comic_date.day() < 9 {
            comic_date_string.push_str("0");
        }
        
        comic_date_string.push_str(&comic_date.day().to_string());


        let main_url: String = String::from(base_url.to_owned() + &comic_date_string);
        println!("{}", main_url);

        let resp: Response = reqwest::get(&main_url).unwrap();

        assert!(resp.status().is_success(), "Url was not successfully retrieved by client, error code was {}", resp.status());


        let resp_document: Document = Document::from_read(resp).unwrap();

        let mut image_url: String = String::default();

        for node in resp_document.find(Class("img-comic")) {
            for attribute in node.attrs().filter(|f| f.0 == "src") {
            // fs::write("C:\\Users\\zjohnson\\Documents\\dilberts\\dilbert.jpg", attribute.1).expect("Unable to write file");
            image_url.push_str("https:");
            image_url.push_str(attribute.1);
            println!("{}", image_url);
            }
        }

        let mut resp = reqwest::get(&image_url).expect("request failed");
        let mut dilbert_file_path = String::default();

        dilbert_file_path.push_str("C:\\Users\\zjohnson\\Documents\\dilberts\\dilbert-");
        dilbert_file_path.push_str(&comic_date_string);
        dilbert_file_path.push_str(".jpg");

        let mut out = File::create(&dilbert_file_path).expect("failed to create file");
        io::copy(&mut resp, &mut out).expect("failed to copy content");

        comic_date = comic_date.checked_add_signed(Duration::days(1)).unwrap();
        println!("{}",comic_date);

    }
    
}


