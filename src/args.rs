use clap::Parser;
#[derive(Debug, Parser)]
#[clap(version)]
pub struct BacArgs {
    /// please provide the path to the bacdive file
    pub bacdive: String,
    /// please provide the id of the species that you want to look,
    pub id: Option<String>,
    /// please provide the country that you want to look,
    pub countrysearch: Option<String>,
    /// please provide the category1 that you want to look,
    pub category1: Option<String>,
    /// please provide the category2 that you want to look,
    pub catgeory2: Option<String>,
    /// please provide the category3 that you want to look,
    pub category3: Option<String>,
    /// please provide the category that you want to look
    pub category: Option<String>,
    /// this will list all the available countries in the bacdive
    pub countrylist: Option<bool>,
    /// this will list all the available continent specific information in the bacdive
    pub continetlist: Option<bool>,
    /// this will list all the available category1 in the bacdive.
    pub category1list: Option<bool>,
    /// this will list all the available category2 in the bacdive.
    pub category2list: Option<bool>,
    /// this will list all the available category3 in the bacdive.
    pub category3list: Option<bool>,
}
