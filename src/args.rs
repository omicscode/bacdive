use clap::{Parser, Subcommand};
#[derive(Debug, Parser)]
#[command(
    name = "bacdive",
    version = "1.0",
    about = "prepairing the json for the bacdive"
)]
pub struct CommandParse {
    /// subcommands for the specific actions
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// please provide the id of the species that you want to look,
    Id {
        /// please provide the path to the bacdive file
        bacdive: String,
        /// specific ID
        id: String,
    },
    /// please provide the category1 that you want to look,
    Category1 {
        /// please provide the path to the bacdive file
        bacdive: String,
        /// specific category1
        category1: String,
    },
    /// please provide the category2 that you want to look,
    Category2 {
        /// please provide the path to the bacdive file
        bacdive: String,
        /// specific category2
        category2: String,
    },
    /// please provide the category3 that you want to look,
    Category3 {
        /// please provide the path to the bacdive file
        bacdive: String,
        /// specific category3
        category3: String,
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
    Countrylist {
        /// please provide the path to the bacdive file
        bacdive: String,
    },
    /// this will list all the available continent specific information in the bacdive
    Continentlist {
        /// please provide the path to the bacdive file
        bacdive: String,
    },
    /// this will list all the available category1 in the bacdive.
    Category1list {
        /// please provide the path to the bacdive file
        bacdive: String,
    },
    /// this will list all the available category2 in the bacdive.
    Category2list {
        /// please provide the path to the bacdive file
        bacdive: String,
    },
    /// this will list all the available category3 in the bacdive.
    Category3list {
        /// please provide the path to the bacdive file
        bacdive: String,
    },
}
