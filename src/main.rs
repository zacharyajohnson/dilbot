use chrono::{Duration, NaiveDate};
use reqwest::Response;
use select::document::Document;
use select::predicate::Class;
use std::path::PathBuf;
use std::{fs, io, process, thread, time};

// Base url for the dilberts website, where we will
// scrape the comics from
const BASE_URL: &str = "https://dilbert.com/strip/";

// This represents the date when the first Dilbert
// was published.
//
// TODO I wanted to make this variable a NaiveDate,
// but chrono does not have const functions to create one.
// Rust only allows functions to be called to create a const 
// variable if they are const as well.
const FIRST_DILBERT_DATE: &str = "1989-04-16";

// YYYY-MM-DD
// See https://docs.rs/chrono/0.4.0/chrono/format/strftime/index.html
// This is the format that base url expects to get to a specific dilbert comic
// EXAMPLE: BASE_URL/YYYY-MM-DD
const DATE_FORMAT: &str = "%Y-%m-%d";

fn main() {
    get_data();
}

fn get_data() {
    let mut comic_date: NaiveDate = NaiveDate::from_ymd(1989, 4, 16);

    while comic_date != NaiveDate::from_ymd(2019, 10, 15) {
        let comic_date_string: String = comic_date.format(DATE_FORMAT).to_string();

        let main_url: String = String::from(BASE_URL.to_owned() + &comic_date_string);
        println!("{}", main_url);

        let resp: Response = reqwest::get(&main_url).unwrap();

        assert!(
            resp.status().is_success(),
            "Url was not successfully retrieved by client, error code was {}",
            resp.status()
        );

        let resp_document: Document = Document::from_read(resp).unwrap();

        let mut image_url: String = String::default();

        /*
         * I don't like using loop labels, but this is to prevent
         * The image url from being invalid if two comics happen
         * to get released on the same day. I don't think that has
         * ever happened, but it doesn't hurt to try and make
         * the program robust.
         */
       'outer: for node in resp_document.find(Class("img-comic")) {
            for attribute in node.attrs().filter(|f| f.0 == "src") {
                image_url.push_str(attribute.1);
                println!("{}", image_url);
                break 'outer
            }
        }

        let mut resp = reqwest::get(&image_url).expect("request failed");
        let mut dilbert_file_path = match dirs::home_dir() {
            Some(mut home_dir) => {
                home_dir.push("dilberts");
                home_dir
            }

            //TODO Need to handle when home dir isn't found
            //TODO We need to consider also how to save this option when the program is run later.
            //Perhaps a config file?
            None => PathBuf::from("test"), 
        };

        if !dilbert_file_path.as_path().exists() {
            println!(
                "Creating directory {}",
                dilbert_file_path.as_path().to_string_lossy()
            );

            match fs::create_dir_all(dilbert_file_path.as_path()) {
                Ok(_) => println!(
                    "Successfully created directory to store dilberts\n{}",
                    dilbert_file_path.as_path().to_string_lossy()
                ),
                Err(e) => {
                    println!(
                        "Failed to create directory {} to store dilberts\n Cause: {}",
                        dilbert_file_path.as_path().to_string_lossy(),
                        e
                    );
                    process::exit(1);
                }
            }
        }

        dilbert_file_path.set_file_name("dilbert-".to_owned() + &comic_date_string + ".jpg");

        let mut out = fs::File::create(dilbert_file_path.as_path()).expect("failed to create file");
        io::copy(&mut resp, &mut out).expect("failed to copy content");

        comic_date = comic_date.checked_add_signed(Duration::days(1)).unwrap();
        println!("{}", comic_date);
        println!("Sleeping for 5 seconds");
        thread::sleep(time::Duration::from_secs(5));
    }
}
