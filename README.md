# bacdive
- rust bacdive for [metagenome BacDive](https://bacdive.dsmz.de/)
- Isolation tale can be downloaded from here: [isolation table](https://bacdive.dsmz.de/isolation-sources).
- please see the last commit message and if it says compiled binary then it is completed or else still in development version.
- writing code for the plot generation of the selected ids such as [stats](https://bacdive.dsmz.de/dashboard) and integrating into a desktop application using Leptos. 

 ```
 cargo build 
 ```
 
```
 Usage: pangenome-graph-bacdive <BACDIVE> [ID] [COUNTRYSEARCH] [CATEGORY1] [CATGEORY2] [CATEGORY3] \
                 [CATEGORY] [COUNTRYLIST] [CONTINETLIST] [CATEGORY1LIST] [CATEGORY2LIST] [CATEGORY3LIST]

 Arguments:
  <BACDIVE>        please provide the path to the bacdive file
  [ID]             please provide the id of the species that you want to look,
  [COUNTRYSEARCH]  please provide the country that you want to look,
  [CATEGORY1]      please provide the category1 that you want to look,
  [CATGEORY2]      please provide the category2 that you want to look,
  [CATEGORY3]      please provide the category3 that you want to look,
  [CATEGORY]       please provide the category that you want to look
  [COUNTRYLIST]    this will list all the available countries in the bacdive [possible values: true, false]
  [CONTINETLIST]   this will list all the available continent specific information in the bacdive [possible values: true, false]
  [CATEGORY1LIST]  this will list all the available category1 in the bacdive [possible values: true, false]
  [CATEGORY2LIST]  this will list all the available category2 in the bacdive [possible values: true, false]
  [CATEGORY3LIST]  this will list all the available category3 in the bacdive [possible values: true, false]

 Options:
  -h, --help     Print help
  -V, --version  Print version
 ```

Gaurav Sablok
