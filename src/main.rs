mod args;
use crate::args::CommandParse;
use crate::args::Commands;
use clap::Parser;
use serde::Serialize;
use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
mod designation;
mod designation;
mod idlist;
mod idsearch;
mod idwrite;
mod species;
mod specieslist;
mod specieswrite;
mod strain;
mod strainheader;
mod strainnumber;
mod strainwrite;
mod structfile;
mod uniqueid;
mod uniquespecies;
mod uniquestrain;
use crate::designation::bacdivedesignationsearch;
use crate::idsearch::bacdiveidsearch;
use crate::species::bacdivespeciessearch;
use crate::strain::bacdivestrainsearch;

/*

 Author Gaurav Sablok
 SLB Potsdam
 Date 2024-1-24

bacdiverust - analyze the microbial genotypes using the
rust-bacdive standalone. It will prepare all the files for the json API for the bacdive as well as the bacdive for the sql insertion.

*/

fn main() {
    let bacdiveargs = CommandParse::parse();
    match &bacdiveargs.command {
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
        Commands::Strain { bacdive, strain } => {
            let commandoutput = strain_write(bacdive, strain).unwrap();
            println!(
                "The strain specific information are as follows:{:?}",
                commandoutput
            );
        }
        Commands::IdList { bacdive } => {
            let commandoutput = unique_id(bacdive).unwrap();
            println!("The category2 searches are: {:?}", commandoutput);
        }
        Commands::SpeciesList { bacdive } => {
            let commandoutput = unique_species(bacdive).unwrap();
            println!("The category2 searches are: {:?}", commandoutput);
        }
        Commands::Strainlist { bacdive } => {
            let commandoutput = unique_strain(bacdive).unwrap();
            println!("The category2 searches are: {:?}", commandoutput);
        }
        Commands::IDListAnalyze { bacdive_analyzer } => {
            let commandoutput: HashSet<String> = idlist(bacdive_analyzer).unwrap();
            println!("The ids present in the bacdive are: {:?}", commandoutput);
        }
        Commands::SpeciesListAnalyze { bacdive_analyzer } => {
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
            let commandoutput = bacdiveidsearch(bacdive_analyzer, id.clone()).unwrap();
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
            let commandoutput = bacdivespeciessearch(bacdive_analyzer, species.clone()).unwrap();
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
            let commandoutput =
                bacdivedesignationsearch(bacdive_analyzer, designation.clone()).unwrap();
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
            let commandoutput = bacdivestrainsearch(bacdive_analyzer, strain.clone()).unwrap();
            for i in commandoutput.iter() {
                println!(
                "The id of the species is:{:?}\nThe species number is {:?}\nThe designation header is: {:?}\n",i.id, i.species, i.speciesinformation
            );
            }
        }
    }
}
