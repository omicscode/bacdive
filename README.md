# pangenome-graph-bacdive
- This analyses the isolates table from the [BacDive](https://bacdive.dsmz.de/) and return json for the rest api.
- the second part is bacdive-analyzer, which integrates all the microbial and metagenome analysis.
- last part is the docker based rust-bacdive, which ports a bacdive straight to your computer, so that you can analyze everything as a standlaone application.
- please see the last commit message and if it says compiled binary then it is completed or else still in development version.

 ```
 cargo build 
 ```
 
```
 prepairing the json for the bacdive

 Usage: bacdive <COMMAND>

 Commands:
  id             please provide the id of the species that you want to look,
  countrysearch  please provide the country that you want to look,
  category1      please provide the category1 that you want to look,
  category2      please provide the category2 that you want to look,
  category3      please provide the category3 that you want to look,
  id-list        this will list all the available unique ids present in the bacdive
  species-list   this will list all the unique species present in the bacdive
  countrylist    this will list all the available countries in the bacdive
  continentlist  this will list all the available continent specific information in the bacdive
  category1list  this will list all the available category1 in the bacdive
  category2list  this will list all the available category2 in the bacdive
  category3list  this will list all the available category3 in the bacdive
  help           Print this message or the help of the given subcommand(s)

 Options:
  -h, --help     Print help
  -V, --version  Print version

```

- to get the subcommand 

```
 please provide the id of the species that you want to look,

Usage: bacdive id <BACDIVE> <ID>

Arguments:
  <BACDIVE>  please provide the path to the bacdive file
  <ID>       specific ID

Options:
  -h, --help  Print help
➜  bacdive git:(main) ✗ ./target/debug/bacdive category1 -h
please provide the category1 that you want to look,

Usage: bacdive category1 <BACDIVE> <CATEGORY1>

Arguments:
  <BACDIVE>    please provide the path to the bacdive file
  <CATEGORY1>  specific category1

Options:
  -h, --help  Print help

```

Gaurav Sablok
