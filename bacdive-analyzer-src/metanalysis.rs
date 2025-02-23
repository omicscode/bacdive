mod args;
mod designation;
mod species;
mod strain;
mod idsearch;
use crate::args::CommandParse;
use crate::args::Commands;
use crate::idsearch::bacdiveidsearch;
use crate::species::bacdivespeciessearch;
use crate::strain::bacdivestrainsearch;
use crate::designation::bacdivedesignationsearch;
use clap::Parser;
use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

/*
*Author Gaurav Sablok
*Universitat Potsdam and SLB Potsdam
*Date 2024-1-27
bacdive analyzer - this analyzes the bacdive isolates and
provides a approach for the identification and retrieval
of the isolates and also allows for the comparative analysis.

*/

fn main() {
    let bacdiveargs = CommandParse::parse();
    match &bacdiveargs.command {
        Commands::IDList { bacdive_analyzer } => {
            let commandoutput: HashSet<String> = idlist(bacdive_analyzer).unwrap();
            println!("The ids present in the bacdive are: {:?}", commandoutput);
        }
        Commands::SpeciesList { bacdive_analyzer } => {
            let commandoutput: HashSet<String> = species(bacdive_analyzer).unwrap();
            println!(
                "The species present in the bacdive are: {:?}",
                commandoutput
            );
        }
        Commands::DesignationList { bacdive_analyzer } => {
            let commandoutput: HashSet<String> = designation(bacdive_analyzer).unwrap();
            println!(
                "The designation species present in the bacdive are: {:?}",
                commandoutput
            );
        }
        Commands::StrainNumberList { bacdive_analyzer } => {
            let commandoutput: HashSet<String> = strainnumber(bacdive_analyzer).unwrap();
            println!(
                "The strain number are as follows for the species in the bacdive:{:?}",
                commandoutput
            );
        }
        Commands::StrainheaderList { bacdive_analyzer } => {
            let commandoutput: HashSet<String> = strainheader(bacdive_analyzer).unwrap();
            println!(
                "The strain header are as follows for the bacdive:{:?}",
                commandoutput
            )
        }
        Commands::IDSearch {
            bacdive_analyzer,
            id,
        } => {
            let commandoutput = bacdiveidsearch(
                bacdive_analyzer,
                id.clone(),
            )
            .unwrap();
            for i in commandoutput.iter() {
                println!(
                "The id of the species is:{:?}\nThe species number is {:?}\nThe designation header is: {:?}\n",i.id, i.species, i.speciesinformation
            );
            }
        }
        Commands::SpeciesSearch {
            bacdive_analyzer,
            species,
        } => {
            let commandoutput = bacdivespeciessearch(
                bacdive_analyzer,
                species.clone(),
            )
            .unwrap();
            for i in commandoutput.iter() {
                println!(
                "The id of the species is:{:?}\nThe species number is {:?}\nThe designation header is: {:?}\n",i.id, i.species, i.speciesinformation
            );
            }
        }
        Commands::DesignationSearch {
            bacdive_analyzer,
            designation,
        } => {
            let commandoutput = bacdivedesignationsearch(
                bacdive_analyzer,
                designation.clone(),
            )
            .unwrap();
            for i in commandoutput.iter() {
                println!(
                "The id of the species is:{:?}\nThe species number is {:?}\nThe designation header is: {:?}\n",i.id, i.species, i.speciesinformation
            );
            }
        }
        Commands::StrainSearch {
            bacdive_analyzer,
            strain,
        } => {
            let commandoutput = bacdivestrainsearch(
                bacdive_analyzer,
                strain.clone(),
            )
            .unwrap();
            for i in commandoutput.iter() {
                println!(
                "The id of the species is:{:?}\nThe species number is {:?}\nThe designation header is: {:?}\n",i.id, i.species, i.speciesinformation
            );
            }
        }
    }
}

fn idlist(path: &str) -> Result<HashSet<String>, Box<dyn Error>> {
    let fileopen = File::open(path).expect("file not found");
    let fileread = BufReader::new(fileopen);
    let mut uniqueid: HashSet<String> = HashSet::new();
    for i in fileread.lines() {
        let line = i.expect("line not found");
        if line.starts_with("\"") || line.starts_with("ID") || line.is_empty() {
            continue;
        } else if !line.starts_with("\"") || !line.starts_with("ID") {
            let linesplit: Vec<_> = line.split(",").collect::<Vec<_>>();
            uniqueid.insert(linesplit[0].to_string());
        }
    }

    Ok(uniqueid)
}

fn species(path: &str) -> Result<HashSet<String>, Box<dyn Error>> {
    let fileopen = File::open(path).expect("file not present");
    let fileread = BufReader::new(fileopen);
    let mut specieshash: HashSet<String> = HashSet::new();
    for i in fileread.lines() {
        let line = i.expect("line not found");
        if line.starts_with("\"") || line.starts_with("ID") || line.is_empty() {
            continue;
        } else if !line.starts_with("\"") || !line.starts_with("ID") || !line.is_empty() {
            specieshash.insert(
                line.split(",")
                    .map(|x| x.replace("\"", ""))
                    .collect::<Vec<_>>()[1]
                    .split(",")
                    .collect::<Vec<_>>()
                    .join("-")
                    .to_string(),
            );
        }
    }
    Ok(specieshash)
}

fn designation(path: &str) -> Result<HashSet<String>, Box<dyn Error>> {
    let fileopen = File::open(path).expect("file not found");
    let fileread = BufReader::new(fileopen);
    let mut header: HashSet<String> = HashSet::new();
    for i in fileread.lines() {
        let line = i.expect("line not found");
        if line.starts_with("\"") || line.starts_with("ID") || line.is_empty() {
            continue;
        } else if !line.starts_with("\"") || !line.starts_with("ID") || !line.is_empty() {
            header.insert(
                line.split(",")
                    .map(|x| x.replace("\"", ""))
                    .collect::<Vec<_>>()[2]
                    .split(" ")
                    .collect::<Vec<_>>()
                    .join("-")
                    .to_string(),
            );
        }
    }
    Ok(header)
}

fn strainnumber(path: &str) -> Result<HashSet<String>, Box<dyn Error>> {
    let fileopen = File::open(path).expect("file not found");
    let fileread = BufReader::new(fileopen);
    let mut strain: HashSet<String> = HashSet::new();
    for i in fileread.lines() {
        let line = i.expect("line not found");
        if line.starts_with("\"") || line.starts_with("ID") || line.is_empty() {
            continue;
        } else if !line.starts_with("\"") || !line.starts_with("ID") || !line.is_empty() {
            let lineselect = &line
                .split(",")
                .map(|x| x.replace("\"", ""))
                .collect::<Vec<_>>()[3];
            strain.insert(lineselect.to_string());
            let lineselectadd = &line
                .split(",")
                .map(|x| x.replace("\"", ""))
                .collect::<Vec<_>>()[4];
            if lineselectadd != "0" {
                strain.insert(lineselect.to_string());
            }
        }
    }

    Ok(strain)
}

fn strainheader(path: &str) -> Result<HashSet<String>, Box<dyn Error>> {
    let fileopen = File::open(path).expect("file not found");
    let fileread = BufReader::new(fileopen);
    let mut strainheader: HashSet<String> = HashSet::new();
    for i in fileread.lines() {
        let line = i.expect("file not found");
        if line.starts_with("\"") || line.starts_with("ID") || line.is_empty() {
            continue;
        } else if !line.starts_with("\"") || !line.starts_with("\"") || !line.is_empty() {
            let header = &line
                .split(",")
                .map(|x| x.replace("\"", ""))
                .collect::<Vec<_>>()[4];
            if header.clone() == "0" || header.clone() == "1" {
                strainheader.insert(header.to_string());
            }
        }
    }
    Ok(strainheader)
}
