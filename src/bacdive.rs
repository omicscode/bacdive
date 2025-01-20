/*
 * holding all the structs in the separate files so that they
 * can be easily called as a reference call in the result.
 *
 * */


#[derive(Debug, Clone, PartialOrd, PartialEq)]
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

#[derive(Debug, Clone, PartialOrd, PartialEq)]
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









