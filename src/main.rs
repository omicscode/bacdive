mod args;
mod bacdive;
use crate::args::CommandParse;
use crate::args::Commands;
use crate::bacdive::BacdiveFilter;
use clap::Parser;
use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

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
    let bacdiveargs  = CommandParse::parse();
    match &bacdiveargs.command {
        Commands::Id { bacdive , id  } => {
          let commandoutput = id_write(bacdive, id).unwrap();
          println!("The ids are: {:?}", commandoutput);
        }
         Commands::Category1 {bacdive, category1} => {
          let commandoutput = category1_write(bacdive, category1).unwrap();
             println!("The category1 are as follow: {:?}", commandoutput);
         }
             Commands::Category2 {bacdive, category2} => {
          let commandoutput = category2_write(bacdive, category2).unwrap();
          println!("The category2 searches are: {:?}", commandoutput);
         }
          Commands::Category3 {bacdive, category3} => {
          let commandoutput = category2_write(bacdive,category3).unwrap();
          println!("The category2 searches are: {:?}", commandoutput);
         }
         Commands::IdList {bacdive} => {
          let commandoutput = unique_id(bacdive).unwrap();
          println!("The category2 searches are: {:?}", commandoutput);
         }
         Commands::SpeciesList {bacdive} => {
          let commandoutput = unique_species(bacdive).unwrap();
          println!("The category2 searches are: {:?}", commandoutput);
         }
         Commands::Countrylist {bacdive} => {
          let commandoutput = unique_country(bacdive).unwrap();
          println!("The category2 searches are: {:?}", commandoutput);
         }
          Commands::Continentlist {bacdive} => {
          let commandoutput = unique_continent(bacdive).unwrap();
          println!("The category2 searches are: {:?}", commandoutput);
         }
          Commands::Category1list {bacdive} => {
          let commandoutput = unique_category1(bacdive).unwrap();
          println!("The category2 searches are: {:?}", commandoutput);
         }
          Commands::Category2list {bacdive} => {
          let commandoutput = unique_category2(bacdive).unwrap();
          println!("The category2 searches are: {:?}", commandoutput);
         }
          Commands::Category3list {bacdive} => {
          let commandoutput = unique_category3(bacdive).unwrap();
          println!("The category2 searches are: {:?}", commandoutput);
         }

     }
   }

fn unique_id(path: &str) -> Result<HashSet<String>, Box<dyn Error>> {
    let mut bacstring: Vec<String> = Vec::new();
        let bacopen = File::open(path).expect("file not found");
        let bacread = BufReader::new(bacopen);
        for i in bacread.lines() {
            let line = i.expect("line not found");
            bacstring.push(line);
        }
    let mut uniqueid: HashSet<String> = HashSet::new();
    for i in bacstring.iter() {
        let countryline:Vec<_> = i.split(",").collect::<Vec<_>>();
        uniqueid.insert(countryline[0].to_string());
    }

    Ok(uniqueid)
}

fn unique_country(path: &str) -> Result<HashSet<String>, Box<dyn Error>> {
    let mut bacstring: Vec<String> = Vec::new();
        let bacopen = File::open(path).expect("file not found");
        let bacread = BufReader::new(bacopen);
        for i in bacread.lines() {
            let line = i.expect("line not found");
            bacstring.push(line);
        }
    let mut uniquecountry: HashSet<String> = HashSet::new();
    for i in bacstring.iter() {
        let countryline = i.split(",").collect::<Vec<_>>();
        uniquecountry.insert(countryline[4].to_string());
    }

    Ok(uniquecountry)
}

fn unique_category1(path: &str) -> Result<HashSet<String>, Box<dyn Error>> {
    let mut bacstring: Vec<String> = Vec::new();
        let bacopen = File::open(path).expect("file not found");
        let bacread = BufReader::new(bacopen);
        for i in bacread.lines() {
            let line = i.expect("line not found");
            bacstring.push(line);
        }
    let mut uniquecategory1: HashSet<String> = HashSet::new();
    for i in bacstring.iter() {
        let uniquecategory: String = i.split(",").collect::<Vec<_>>()[6]
            .replace("#", "")
            .to_string();
        uniquecategory1.insert(uniquecategory);
    }
    Ok(uniquecategory1)
}

fn unique_category2(path: &str) -> Result<HashSet<String>, Box<dyn Error>> {
    let mut bacstring: Vec<String> = Vec::new();
        let bacopen = File::open(path).expect("file not found");
        let bacread = BufReader::new(bacopen);
        for i in bacread.lines() {
            let line = i.expect("line not found");
            bacstring.push(line);
        }
    let mut uniquecategory2: HashSet<String> = HashSet::new();
    for i in bacstring.iter() {
        let uniquecategory: String = i.split(",").collect::<Vec<_>>()[7]
            .replace("#", "")
            .to_string();
        uniquecategory2.insert(uniquecategory);
    }
    Ok(uniquecategory2)
}


fn unique_category3(path: &str) -> Result<HashSet<String>, Box<dyn Error>> {
    let mut bacstring: Vec<String> = Vec::new();
        let bacopen = File::open(path).expect("file not found");
        let bacread = BufReader::new(bacopen);
        for i in bacread.lines() {
            let line = i.expect("line not found");
            bacstring.push(line);
        }
    let mut uniquecategory3: HashSet<String> = HashSet::new();
    for i in bacstring.iter() {
        let uniquecategory: String = i.split(",").collect::<Vec<_>>()[8]
            .replace("#", "")
            .to_string();
        uniquecategory3.insert(uniquecategory);
    }
    Ok(uniquecategory3)
    }

fn unique_continent(path: &str) -> Result<HashSet<String>, Box<dyn Error>> {
    let mut bacstring: Vec<String> = Vec::new();
        let bacopen = File::open(path).expect("file not found");
        let bacread = BufReader::new(bacopen);
        for i in bacread.lines() {
            let line = i.expect("line not found");
            bacstring.push(line);
        }
    let mut uniquecontinent: HashSet<String> = HashSet::new();
    for i in bacstring.iter() {
        let countryline:Vec<_> = i.split(",").collect::<Vec<_>>();
        uniquecontinent.insert(countryline[5].to_string());
    }

    Ok(uniquecontinent)
}

fn unique_species(path: &str) -> Result<HashSet<String>, Box<dyn Error>> {
    let mut bacstring: Vec<String> = Vec::new();
        let bacopen = File::open(path).expect("file not found");
        let bacread = BufReader::new(bacopen);
        for i in bacread.lines() {
            let line = i.expect("line not found");
            bacstring.push(line);
        }
    let mut uniquespecies: HashSet<String> = HashSet::new();
    for i in bacstring.iter() {
        let species:Vec<_> = i.split(",").collect::<Vec<_>>();
        uniquespecies.insert(species[5].to_string());
    }

    Ok(uniquespecies)
}




fn species_write(path: &str, id: &str) -> Result<Vec<BacdiveFilter>, Box<dyn Error>> {
    let mut bacstring: Vec<String> = Vec::new();
    let bacopen = File::open(path).expect("file not found");
    let bacread = BufReader::new(bacopen);
    for i in bacread.lines() {
         let line = i.expect("line nout found");
         bacstring.push(line);
    }
    let mut bachold: Vec<BacdiveFilter> = Vec::new();
    for i in bacstring.iter() {
        let itersline:Vec<_> = i.split(",").collect::<Vec<_>>();
        if itersline[1] == id {
            let mut cat1hold: Vec<String> = Vec::new();
            let mut cat2hold: Vec<String> = Vec::new();
            let mut cat3hold: Vec<String> = Vec::new();
            let mut countryhold: Vec<String> = Vec::new();
            let mut continenthold: Vec<String> = Vec::new();
            cat1hold.push(itersline[6].to_string());
            cat2hold.push(itersline[7].to_string());
            cat3hold.push(itersline[8].to_string());
            countryhold.push(itersline[4].to_string());
            continenthold.push(itersline[5].to_string());
            bachold.push(BacdiveFilter {
                id: itersline[0].to_string(),
                species: itersline[1].to_string(),
                collectionnumber: itersline[2].to_string(),
                isolationsource: itersline[3].to_string(),
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

fn id_write(path: &str, id: &str) -> Result<Vec<BacdiveFilter>, Box<dyn Error>> {
    let mut bacstring: Vec<String> = Vec::new();
    let bacopen = File::open(path).expect("file not found");
    let bacread = BufReader::new(bacopen);
    for i in bacread.lines() {
         let line = i.expect("line nout found");
         bacstring.push(line);
    }
    let mut bachold: Vec<BacdiveFilter> = Vec::new();
    for i in bacstring.iter() {
        let itersline:Vec<_> = i.split(",").collect::<Vec<_>>();
        if itersline[0] == id {
            let mut cat1hold: Vec<String> = Vec::new();
            let mut cat2hold: Vec<String> = Vec::new();
            let mut cat3hold: Vec<String> = Vec::new();
            let mut countryhold: Vec<String> = Vec::new();
            let mut continenthold: Vec<String> = Vec::new();
            cat1hold.push(itersline[6].to_string());
            cat2hold.push(itersline[7].to_string());
            cat3hold.push(itersline[8].to_string());
            countryhold.push(itersline[4].to_string());
            continenthold.push(itersline[5].to_string());
            bachold.push(BacdiveFilter {
                id: itersline[0].to_string(),
                species: itersline[1].to_string(),
                collectionnumber: itersline[2].to_string(),
                isolationsource: itersline[3].to_string(),
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


fn category1_write(path: &str, category1: &str) -> Result<Vec<BacdiveFilter>, Box<dyn Error>> {
    let mut bacstring: Vec<String> = Vec::new();
    let bacopen = File::open(path).expect("file not found");
    let bacread = BufReader::new(bacopen);
    for i in bacread.lines() {
         let line = i.expect("line nout found");
         bacstring.push(line);
    }
    let mut bachold: Vec<BacdiveFilter> = Vec::new();
    for i in bacstring.iter() {
        let itersline:Vec<_> = i.split(",").collect::<Vec<_>>();
        if itersline[6] == category1 {
            let mut cat1hold: Vec<String> = Vec::new();
            let mut cat2hold: Vec<String> = Vec::new();
            let mut cat3hold: Vec<String> = Vec::new();
            let mut countryhold: Vec<String> = Vec::new();
            let mut continenthold: Vec<String> = Vec::new();
            cat1hold.push(itersline[6].to_string());
            cat2hold.push(itersline[7].to_string());
            cat3hold.push(itersline[8].to_string());
            countryhold.push(itersline[4].to_string());
            continenthold.push(itersline[5].to_string());
            bachold.push(BacdiveFilter {
                id: itersline[0].to_string(),
                species: itersline[1].to_string(),
                collectionnumber: itersline[2].to_string(),
                isolationsource: itersline[3].to_string(),
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


fn category2_write(path: &str, category2: &str) -> Result<Vec<BacdiveFilter>, Box<dyn Error>> {
    let mut bacstring: Vec<String> = Vec::new();
    let bacopen = File::open(path).expect("file not found");
    let bacread = BufReader::new(bacopen);
    for i in bacread.lines() {
         let line = i.expect("line nout found");
         bacstring.push(line);
    }
    let mut bachold: Vec<BacdiveFilter> = Vec::new();
    for i in bacstring.iter() {
        let itersline:Vec<_> = i.split(",").collect::<Vec<_>>();
        if itersline[7] == category2 {
            let mut cat1hold: Vec<String> = Vec::new();
            let mut cat2hold: Vec<String> = Vec::new();
            let mut cat3hold: Vec<String> = Vec::new();
            let mut countryhold: Vec<String> = Vec::new();
            let mut continenthold: Vec<String> = Vec::new();
            cat1hold.push(itersline[6].to_string());
            cat2hold.push(itersline[7].to_string());
            cat3hold.push(itersline[8].to_string());
            countryhold.push(itersline[4].to_string());
            continenthold.push(itersline[5].to_string());
            bachold.push(BacdiveFilter {
                id: itersline[0].to_string(),
                species: itersline[1].to_string(),
                collectionnumber: itersline[2].to_string(),
                isolationsource: itersline[3].to_string(),
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

fn category3_write(path: &str, category3: &str) -> Result<Vec<BacdiveFilter>, Box<dyn Error>> {
    let mut bacstring: Vec<String> = Vec::new();
    let bacopen = File::open(path).expect("file not found");
    let bacread = BufReader::new(bacopen);
    for i in bacread.lines() {
         let line = i.expect("line nout found");
         bacstring.push(line);
    }
    let mut bachold: Vec<BacdiveFilter> = Vec::new();
    for i in bacstring.iter() {
        let itersline:Vec<_> = i.split(",").collect::<Vec<_>>();
        if itersline[8] == category3 {
            let mut cat1hold: Vec<String> = Vec::new();
            let mut cat2hold: Vec<String> = Vec::new();
            let mut cat3hold: Vec<String> = Vec::new();
            let mut countryhold: Vec<String> = Vec::new();
            let mut continenthold: Vec<String> = Vec::new();
            cat1hold.push(itersline[6].to_string());
            cat2hold.push(itersline[7].to_string());
            cat3hold.push(itersline[8].to_string());
            countryhold.push(itersline[4].to_string());
            continenthold.push(itersline[5].to_string());
            bachold.push(BacdiveFilter {
                id: itersline[0].to_string(),
                species: itersline[1].to_string(),
                collectionnumber: itersline[2].to_string(),
                isolationsource: itersline[3].to_string(),
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
