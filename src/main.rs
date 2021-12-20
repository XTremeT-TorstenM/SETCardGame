#![allow(dead_code, unused_variables, unused_imports)]

use rand::prelude::*;
use reqwest::StatusCode;
use scraper::{Html, Selector};

mod utils;

#[tokio::main]
async fn main() {

    let mut index = 1;
    let mut variations12 = vec![vec![1; 3]; 220];
    let konarr12 = vec![1,2,3,4,5,6,7,8,9,10,11,12];
    let mut cardarray = vec![vec![0; 4]; 12];

    for j in 0..konarr12.len() - 2 {
        for k in 1..konarr12.len() - 1 {
            for l in 2..konarr12.len() {
                if l <= k || l <= j || k <= j {continue;};
                variations12[index-1][0] = konarr12[j];
                variations12[index-1][1] = konarr12[k];
                variations12[index-1][2] = konarr12[l];
                index += 1;
            }
        }
    }

    let client = utils::get_client();
    let url = "https://www.setgame.com/set/puzzle";
    let result = client.get(url).send().await.unwrap();

    let raw_html = match result.status() {
        StatusCode::OK => result.text().await.unwrap(),
        _ => panic!("Something went horrible wrong"),
    };

    for i in 1..=12 {
        let tststr = String::from("img.A".to_owned() + &i.to_string());
        let document = Html::parse_document(&raw_html);
        let article_selector = Selector::parse(&tststr).unwrap();

        for element in document.select(&article_selector) {
            let src = match element.value().attr("src") {
                Some(img_src) => img_src,
                _ => "no img src found",
            };
            let nmbpng = &src[49..];
            let nmb: i32 = nmbpng.replace(".png", "").parse().unwrap(); 

            let mut anz = nmb % 3;
            if  anz == 0 {anz = 3};
            cardarray[i-1][0] = anz;
            
            match nmb {
                1..=27 => cardarray[i-1][2] = 1,
                28..=54 => cardarray[i-1][2] = 2,
                55..=81 => cardarray[i-1][2] = 3,
                _ => println!("Error"),
            };

            match nmb {
                1..=9 | 28..=36 | 55..=63 => cardarray[i-1][1] = 1,
                10..=18 | 37..=45 | 64..=72 => cardarray[i-1][1] = 2,
                19..=27 | 46..=54 | 73..=81 => cardarray[i-1][1] = 3,
                _ => println!("Error"),
            };
            
            match nmb {
                1..=3 | 10..=12 | 19..=21 | 28..=30 | 37..=39 | 46..=48 | 55..=57 | 64..=66 | 73..=75 => cardarray[i-1][3] = 1,
                4..=6 | 13..=15 | 22..=24 | 31..=33 | 40..=42 | 49..=51 | 58..=60 | 67..=69 | 76..=78 => cardarray[i-1][3] = 2,
                7..=9 | 16..=18 | 25..=27 | 34..=36 | 43..=45 | 52..=54 | 61..=63 | 70..=72 | 79..=81 => cardarray[i-1][3] = 3,
                _ => println!("Error"),
            };
        }
    }

    index = 1;
    for i in 0..220 {
        let x = (variations12[i][0]) - 1;
        let y = (variations12[i][1]) - 1;
        let z = (variations12[i][2]) - 1;

        let a = cardarray[x][0] + cardarray[y][0] + cardarray[z][0];
        let b = cardarray[x][1] + cardarray[y][1] + cardarray[z][1];
        let c = cardarray[x][2] + cardarray[y][2] + cardarray[z][2];
        let d = cardarray[x][3] + cardarray[y][3] + cardarray[z][3];

        if a % 3 != 0 {continue;};
        if b % 3 != 0 {continue;};
        if c % 3 != 0 {continue;};
        if d % 3 != 0 {continue;};

        print!("Lösung {} Karten: {} - {} - {}", index, x + 1, y + 1, z + 1);
        index += 1;
    
        println!("");
    }
}
 
