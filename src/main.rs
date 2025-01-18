mod args;
mod bacdive;
use crate::args::BacArgs;
use crate::bacdive::BacdiveArgs;
use crate::bacdive::BacdiveFilter;
use clap::Parser;
use rocket::serde::json::to_string;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

/*
*Author Gaurav Sablok
*Universitat Potsdam and SLB Potsdam
*Date 2024-1-18

bacdive analyzer- analyze the microbial genotypes using the
rust-bacdive standalone. It will prepare all the files for the
json API for the bacdive as well as the bacdive for the sql
insertion. I am writing a complete bacdive in RUST after this.

Thank you SLB Potsdam for the wide monitor and i was able to write 
this entirely in 3-4 hours. i thank you all.

*/

fn main() {
     let _args = BacArgs::parse();
    bacdive().unwrap();
    unique_category1().unwrap();
    unique_category2().unwrap();
    unique_category3().unwrap();
    unique_species().unwrap();
    unique_country().unwrap();
}

fn bacdive() -> Result<Vec<BacdiveArgs>, Box<dyn Error>> {
    let args = BacArgs::parse();
    let bacdiveopen = File::open(&args.bacdive).expect("file nout foune");
    let bacread = BufReader::new(bacdiveopen);
    let mut bacdivestruct: Vec<BacdiveArgs> = Vec::new();
    let mut bacdivestringhash: Vec<String> = Vec::new();
    for i in bacread.lines() {
        let line = i.expect("line not present");
        let bacsplit: Vec<_> = line.split(",").collect::<Vec<_>>();
        bacdivestruct.push(BacdiveArgs {
            id: bacsplit[0].to_string(),
            species: bacsplit[1].to_string(),
            collectionnumber: bacsplit[2].to_string(),
            isolationsource: bacsplit[3].to_string(),
            country: bacsplit[4].to_string(),
            continent: bacsplit[5].to_string().replace("#", ""),
            category1: bacsplit[6].to_string().replace("#", ""),
            category2: bacsplit[7].to_string().replace("#", ""),
            category3: bacsplit[8].to_string().replace("#", ""),
        });
        bacdivestringhash.push(line);
    }

    Ok(bacdivestruct)
}

fn bacdive_general_purpose() -> Result<Vec<String>, Box<dyn Error>> {
    let argsbacdive = BacArgs::parse();
    let bacopen = File::open(&argsbacdive.bacdive).expect("file not found");
    let bacread = BufReader::new(bacopen);
    let mut bacstring: Vec<String> = Vec::new();
    for i in bacread.lines() {
        let line = i.expect("line not found");
        bacstring.push(line);
    }
    Ok(bacstring)
}

fn unique_country() -> Result<HashSet<String>, Box<dyn Error>> {
    let bacstring: Vec<String> = bacdive_general_purpose().unwrap();
    let mut uniquecountry: HashSet<String> = HashSet::new();
    for i in bacstring.iter() {
        let countryline = i.split(",").collect::<Vec<_>>();
        uniquecountry.insert(countryline[4].to_string());
    }

    Ok(uniquecountry)
}

fn unique_category1() -> Result<HashSet<String>, Box<dyn Error>> {
    let bacstring: Vec<String> = bacdive_general_purpose().unwrap();
    let mut uniquecategory1: HashSet<String> = HashSet::new();
    for i in bacstring.iter() {
        let uniquecategory: String = i.split(",").collect::<Vec<_>>()[6]
            .replace("#", "")
            .to_string();
        uniquecategory1.insert(uniquecategory);
    }
    Ok(uniquecategory1)
}

fn unique_category2() -> Result<HashSet<String>, Box<dyn Error>> {
    let bacstring: Vec<String> = bacdive_general_purpose().unwrap();
    let mut uniquecategory2: HashSet<String> = HashSet::new();
    for i in bacstring.iter() {
        let uniquecategory: String = i.split(",").collect::<Vec<_>>()[7]
            .replace("#", "")
            .to_string();
        uniquecategory2.insert(uniquecategory);
    }
    Ok(uniquecategory2)
}

fn unique_category3() -> Result<HashSet<String>, Box<dyn Error>> {
    let bacstring: Vec<String> = bacdive_general_purpose().unwrap();
    let mut uniquecategory3: HashSet<String> = HashSet::new();
    for i in bacstring.iter() {
        let uniquecategory: String = i.split(",").collect::<Vec<_>>()[8]
            .replace("#", "")
            .to_string();
        uniquecategory3.insert(uniquecategory);
    }
    Ok(uniquecategory3)
}

fn unique_species() -> Result<HashSet<String>, Box<dyn Error>> {
    let bacstring: Vec<String> = bacdive_general_purpose().unwrap();
    let mut uniquespecies: HashSet<String> = HashSet::new();
    for i in bacstring.iter() {
        let unique: String = i.split(",").collect::<Vec<_>>()[4].to_string();
        uniquespecies.insert(unique);
    }
    Ok(uniquespecies)
}

fn species_write(id: &str) -> Result<Vec<BacdiveFilter>, Box<dyn Error>> {
    let bacdive_call = bacdive().unwrap();
    let mut bachold: Vec<BacdiveFilter> = Vec::new();
    for i in bacdive_call.iter() {
        if i.species == id {
            let mut cat1hold: Vec<String> = Vec::new();
            let mut cat2hold: Vec<String> = Vec::new();
            let mut cat3hold: Vec<String> = Vec::new();
            let mut countryhold: Vec<String> = Vec::new();
            let mut continenthold: Vec<String> = Vec::new();
            cat1hold.push(i.category1.clone());
            cat2hold.push(i.category2.clone());
            cat3hold.push(i.category3.clone());
            countryhold.push(i.country.clone());
            continenthold.push(i.continent.clone());
            bachold.push(BacdiveFilter {
                id: i.id.clone(),
                species: i.species.clone(),
                collectionnumber: i.collectionnumber.clone(),
                isolationsource: i.isolationsource.clone(),
                country: countryhold,
                continent: continenthold,
                category1: cat1hold,
                category2: cat2hold,
                category3: cat3hold,
            });
        }
    }

    Ok(bachold)
}

fn country_write(count: &str) -> Result<Vec<BacdiveFilter>, Box<dyn Error>> {
    let bacdive_call = bacdive().unwrap();
    let mut baccountry: Vec<BacdiveFilter> = Vec::new();
    for i in bacdive_call.iter() {
        if i.country == count {
            let mut cat1hold: Vec<String> = Vec::new();
            let mut cat2hold: Vec<String> = Vec::new();
            let mut cat3hold: Vec<String> = Vec::new();
            let mut countryhold: Vec<String> = Vec::new();
            let mut continenthold: Vec<String> = Vec::new();
            cat1hold.push(i.category1.clone());
            cat2hold.push(i.category2.clone());
            cat3hold.push(i.category3.clone());
            countryhold.push(i.country.clone());
            continenthold.push(i.continent.clone());
            baccountry.push(BacdiveFilter {
                id: i.id.clone(),
                species: i.species.clone(),
                collectionnumber: i.collectionnumber.clone(),
                isolationsource: i.isolationsource.clone(),
                country: countryhold,
                continent: continenthold,
                category1: cat1hold,
                category2: cat2hold,
                category3: cat3hold,
            });
        }
    }
    Ok(baccountry)
}

fn category1(item: &str) -> Result<Vec<BacdiveFilter>, Box<dyn Error>> {
    let bacdive_call = bacdive().unwrap();
    let mut baccategory1: Vec<BacdiveFilter> = Vec::new();
    for i in bacdive_call.iter() {
        if i.category1 == item {
            let mut cat1hold: Vec<String> = Vec::new();
            let mut cat2hold: Vec<String> = Vec::new();
            let mut cat3hold: Vec<String> = Vec::new();
            let mut countryhold: Vec<String> = Vec::new();
            let mut continenthold: Vec<String> = Vec::new();
            cat1hold.push(i.category1.clone());
            cat2hold.push(i.category2.clone());
            cat3hold.push(i.category3.clone());
            countryhold.push(i.country.clone());
            continenthold.push(i.continent.clone());
            baccategory1.push(BacdiveFilter {
                id: i.id.clone(),
                species: i.species.clone(),
                collectionnumber: i.collectionnumber.clone(),
                isolationsource: i.isolationsource.clone(),
                country: countryhold,
                continent: continenthold,
                category1: cat1hold,
                category2: cat2hold,
                category3: cat3hold,
            });
        }
    }
    Ok(baccategory1)
}

fn category2(item: &str) -> Result<Vec<BacdiveFilter>, Box<dyn Error>> {
    let bacdive_call = bacdive().unwrap();
    let mut baccategory2: Vec<BacdiveFilter> = Vec::new();
    for i in bacdive_call.iter() {
        if i.category2 == item {
            let mut cat1hold: Vec<String> = Vec::new();
            let mut cat2hold: Vec<String> = Vec::new();
            let mut cat3hold: Vec<String> = Vec::new();
            let mut countryhold: Vec<String> = Vec::new();
            let mut continenthold: Vec<String> = Vec::new();
            cat1hold.push(i.category1.clone());
            cat2hold.push(i.category2.clone());
            cat3hold.push(i.category3.clone());
            countryhold.push(i.country.clone());
            continenthold.push(i.continent.clone());
            baccategory2.push(BacdiveFilter {
                id: i.id.clone(),
                species: i.species.clone(),
                collectionnumber: i.collectionnumber.clone(),
                isolationsource: i.isolationsource.clone(),
                country: countryhold,
                continent: continenthold,
                category1: cat1hold,
                category2: cat2hold,
                category3: cat3hold,
            });
        }
    }
    Ok(baccategory2)
}

fn category(item: &str) -> Result<Vec<BacdiveFilter>, Box<dyn Error>> {
    let bacdive_call = bacdive().unwrap();
    let mut baccategory3: Vec<BacdiveFilter> = Vec::new();
    for i in bacdive_call.iter() {
        if i.category3 == item {
            let mut cat1hold: Vec<String> = Vec::new();
            let mut cat2hold: Vec<String> = Vec::new();
            let mut cat3hold: Vec<String> = Vec::new();
            let mut countryhold: Vec<String> = Vec::new();
            let mut continenthold: Vec<String> = Vec::new();
            cat1hold.push(i.category1.clone());
            cat2hold.push(i.category2.clone());
            cat3hold.push(i.category3.clone());
            countryhold.push(i.country.clone());
            continenthold.push(i.continent.clone());
            baccategory3.push(BacdiveFilter {
                id: i.id.clone(),
                species: i.species.clone(),
                collectionnumber: i.collectionnumber.clone(),
                isolationsource: i.isolationsource.clone(),
                country: countryhold,
                continent: continenthold,
                category1: cat1hold,
                category2: cat2hold,
                category3: cat3hold,
            });
        }
    }
    Ok(baccategory3)
}
