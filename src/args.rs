use clap::{Parser, Subcommand};
#[derive(Debug, Parser)]
#[command(
    name = "bacdive",
    version = "1.0",
    about = "bacdive-analyzer and prepare json"
)]
pub struct CommandParse {
    /// subcommands for the specific actions
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Id {
        /// please provide the path to the bacdive file
        bacdive: String,
        /// specific ID:
        id: String,
    },
    /// please provide the species that need to be searched.
    Species {
        /// please provide the path to the bacdive file
        bacdive: String,
        /// specific species name. Use the species list with the bacdive file to see the species.
        species: String,
    },
    /// please provide the category2 that you want to look,
    Strain {
        /// please provide the path to the bacdive file
        bacdive: String,
        /// specific strain
        strain: String,
    },
    /// this will list all the available unique ids present in the bacdive
    IdList {
        /// please provide the path to the bacdive file
        bacdive: String,
    },
    /// this will list all the unique species present in the bacdive
    SpeciesList {
        /// please provide the path to the bacdive file
        bacdive: String,
    },
    /// this will list all the available countries in the bacdive
    Strainlist {
        /// please provide the path to the bacdive file
        bacdive: String,
    },
}
