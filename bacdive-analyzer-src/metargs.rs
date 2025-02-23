use clap::{Parser, Subcommand};
#[derive(Debug, Parser)]
#[command(name = "bacdive", version = "1.0")]
pub struct CommandParse {
    /// subcommands for the specific actions
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// present the list of the unique ids present
    IDList {
        /// provide the path to the bacdive file
        bacdive_analyzer: String,
    },
    /// provide the species present in the bacdive
    SpeciesList {
        /// provide the path to the bacdive file
        bacdive_analyzer: String,
    },
    /// provide the designation header present in the bacdive
    DesignationList {
        /// provide the path to the bacdive file
        bacdive_analyzer: String,
    },
    /// provide the strain number present in the bacdive
    StrainNumberList {
        /// provide the path to the bacdive file
        bacdive_analyzer: String,
    },
    /// provide the strain header present in the bacdive
    StrainheaderList {
        /// provide the path to the bacdive file
        bacdive_analyzer: String,
    },
    /// search for the specific id and json output
    IDSearch {
        /// please provide the path to the bacdive file
        bacdive_analyzer: String,
        /// please provide the specific id that you want to look
        id: Option<String>,
    },
    /// search for the specific species and json output
    SpeciesSearch {
       /// please provide the path to the bacdive file
       bacdive_analyzer: String,
       /// please provide the species name
       species: Option<String>
    },
    /// search for the specific designation and json output.
    DesignationSearch{
       /// please provide the path to the bacdive file
       bacdive_analyzer: String,
       /// please provide the designation name
       designation: Option<String>
    },
    /// search for the specific strain and json output
    StrainSearch {
       /// please provide the path to the bacdive file
       bacdive_analyzer: String,
       /// please provide the strain
       strain: Option<String>
    },
}
