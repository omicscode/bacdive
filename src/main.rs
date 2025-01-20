mod args;
use crate::args::CommandParse;
use crate::args::Commands;
use clap::Parser;
use rusqlite::Connection;
use serde::Serialize;
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
    let bacdiveargs = CommandParse::parse();
    match &bacdiveargs.command {
        Commands::Partition { bacdive } => {
            let commandoutput = json_prepare(bacdive).unwrap();
            println!(
                "The json file has been save for the webapi:{:?}",
                commandoutput
            );
        }
        Commands::Id { bacdive, id } => {
            let commandoutput = id_write(bacdive, id).unwrap();
            println!("The ids are: {:?}", commandoutput);
        }
        Commands::Species { bacdive, species } => {
            let commandoutput = species_write(bacdive, species).unwrap();
            println!(
                "The species and the associated information are: {:?}",
                commandoutput
            );
        }
        Commands::Country { bacdive, country } => {
            let commandoutput = country_write(bacdive, country).unwrap();
            println!(
                "The details associated with these country are: {:?}",
                commandoutput
            );
        }
        Commands::Category1 { bacdive, category1 } => {
            let commandoutput = category1_write(bacdive, category1).unwrap();
            println!("The category1 are as follow: {:?}", commandoutput);
        }
        Commands::Category2 { bacdive, category2 } => {
            let commandoutput = category2_write(bacdive, category2).unwrap();
            println!("The category2 searches are: {:?}", commandoutput);
        }
        Commands::Category3 { bacdive, category3 } => {
            let commandoutput = category3_write(bacdive, category3).unwrap();
            println!("The category2 searches are: {:?}", commandoutput);
        }
        Commands::IdList { bacdive } => {
            let commandoutput = unique_id(bacdive).unwrap();
            println!("The category2 searches are: {:?}", commandoutput);
        }
        Commands::SpeciesList { bacdive } => {
            let commandoutput = unique_species(bacdive).unwrap();
            println!("The category2 searches are: {:?}", commandoutput);
        }
        Commands::Countrylist { bacdive } => {
            let commandoutput = unique_country(bacdive).unwrap();
            println!("The category2 searches are: {:?}", commandoutput);
        }
        Commands::Continentlist { bacdive } => {
            let commandoutput = unique_continent(bacdive).unwrap();
            println!("The category2 searches are: {:?}", commandoutput);
        }
        Commands::Category1list { bacdive } => {
            let commandoutput = unique_category1(bacdive).unwrap();
            println!("The category2 searches are: {:?}", commandoutput);
        }
        Commands::Category2list { bacdive } => {
            let commandoutput = unique_category2(bacdive).unwrap();
            println!("The category2 searches are: {:?}", commandoutput);
        }
        Commands::Category3list { bacdive } => {
            let commandoutput = unique_category3(bacdive).unwrap();
            println!("The category2 searches are: {:?}", commandoutput);
        }
        Commands::SQLIntegrate { bacdive } => {
            let commandoutput = rust_backhand(bacdive).unwrap();
            println!(
                "The rust backhand database has been written to the sqlite3: {:?}",
                commandoutput
            );
        }
    }
}


#[derive(Debug, Clone, PartialOrd, PartialEq, Serialize)]
pub struct BacdiveArgs {
    pub id: String,
    pub species: String,
    pub collectionnumber : String,
    pub isolationsource: String,
    pub country: String,
    pub continent: String,
    pub category1: String,
    pub category2: String,
    pub category3: String,
}

#[derive(Debug, Clone, Serialize, PartialOrd, PartialEq)]
pub struct BacdiveFilter {
   pub id: String,
   pub species: String,
   pub collectionnumber: String,
   pub isolationsource: String,
   pub country: Vec<String>,
   pub continent: Vec<String>,
   pub category1:Vec<String>,
   pub category2:Vec<String>,
   pub category3: Vec<String>,
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
        let countryline: Vec<_> = i.split(",").collect::<Vec<_>>();
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
        let countryline: Vec<_> = i.split(",").collect::<Vec<_>>();
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
        let species: Vec<_> = i.split(",").collect::<Vec<_>>();
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
        let itersline: Vec<_> = i.split(",").collect::<Vec<_>>();
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
        let itersline: Vec<_> = i.split(",").collect::<Vec<_>>();
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
        let itersline: Vec<_> = i.split(",").collect::<Vec<_>>();
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
        let itersline: Vec<_> = i.split(",").collect::<Vec<_>>();
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
        let itersline: Vec<_> = i.split(",").collect::<Vec<_>>();
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

fn country_write(path: &str, country: &str) -> Result<Vec<BacdiveFilter>, Box<dyn Error>> {
    let mut bacstring: Vec<String> = Vec::new();
    let bacopen = File::open(path).expect("file not found");
    let bacread = BufReader::new(bacopen);
    for i in bacread.lines() {
        let line = i.expect("line nout found");
        bacstring.push(line);
    }
    let mut bachold: Vec<BacdiveFilter> = Vec::new();
    for i in bacstring.iter() {
        let itersline: Vec<_> = i.split(",").collect::<Vec<_>>();
        if itersline[4] == country {
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

fn json_prepare(bacdive: &str) -> Result<String, Box<dyn Error>> {
    let jsonfile = File::open(bacdive).expect("file not found");
    let jsonread = BufReader::new(jsonfile);
    let mut jsonwrite = File::create("bacdivejson").expect("file not found");
    for i in jsonread.lines() {
        let line = i.expect("line not found");
        let linevec = line.split(",").collect::<Vec<_>>();
        let jsonstring = BacdiveArgs {
            id: linevec[0].to_string(),
            species: linevec[1].to_string(),
            collectionnumber: linevec[2].to_string(),
            isolationsource: linevec[3].to_string(),
            country: linevec[4].to_string(),
            continent: linevec[5].replace("#", "").to_string(),
            category1: linevec[6].replace("#", "").to_string(),
            category2: linevec[7].replace("#", "").to_string(),
            category3: linevec[8].replace("#", "").to_string(),
        };
        let jsonwritemed = serde_json::to_string(&jsonstring).unwrap();
        writeln!(jsonwrite, "{:?}\n", jsonwritemed).expect("line not found");
    }

    Ok("json file have been written for data ingestion into the web api".to_string())
}

fn rust_backhand(bacdive: &str) -> Result<String, Box<dyn Error>> {
    let backhandopen = File::open(bacdive).expect("file not found");
    let backhandread = BufReader::new(backhandopen);
    let mut backhandstring: Vec<_> = Vec::new();

    for i in backhandread.lines() {
        let line = i.expect("file not found");
        backhandstring.push(line);
    }

    let mut backdivesql: Vec<BacdiveArgs> = Vec::new();
    for i in backhandstring.iter() {
        let linevec: Vec<_> = i.split(",").collect::<Vec<_>>();
        backdivesql.push(BacdiveArgs {
            id: linevec[0].to_string(),
            species: linevec[1].to_string(),
            collectionnumber: linevec[2].to_string(),
            isolationsource: linevec[3].to_string(),
            country: linevec[4].replace("#", "").to_string(),
            continent: linevec[5].replace("#", "").to_string(),
            category1: linevec[6].replace("#", "").to_string(),
            category2: linevec[7].replace("#", "").to_string(),
            category3: linevec[8].replace("#", "").to_string(),
        });
    }

    let backdiveconnection = Connection::open("bacdive.db")?;
    let _ = backdiveconnection.execute(
        "CREATE TABLE IF NOT EXITS bacdive_sql(
               ID INTEGER PRIMARYKEY,
               SPECIESID TEXT,
               SPECIES TEXT,
               COLLECTION_NUMBER TEXT,
               ISOLATTION_SOURCE TEXT,
               COUNTRY TEXT,
               CONTINENT TEXT,
               CATEGORY1 TEXT,
               CATEGORY2 TEXT,
               CATEGORY3 TEXT
               )",
        (),
    );

    for i in backdivesql.iter() {
        backdiveconnection.execute(
            "INSERT INTO bacdive_sql(SPECIESID, SPECIFC, COLLECTION_NUMBER, ISOLATION_SOURCE,
               COUNTRY, CONTINENT, CATEGORY1, CATEGORY2, CATEGORY3)
                 values(?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            (
                &i.id,
                &i.species,
                &i.collectionnumber,
                &i.isolationsource,
                &i.country,
                &i.continent,
                &i.category1,
                &i.category2,
                &i.category3,
            ),
        )?;
    }

    Ok("rust banhand has been written to the sqlite3 database".to_string())
}
