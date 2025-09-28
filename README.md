# bacdive

- Bacdive:Added a complete new webminer and submitting for publication. A complete RUST async analyzer for bacdive database.
- This analyses the isolates table from the [BacDive](https://bacdive.dsmz.de/).
- Now this is a complete API RUST based for the BacDive and analyzes all of them asynchronously.

 ```
 cargo build
 ```

 ```
 _                      ____    ___  __     __  _____
 _                      ____    ___  __     __  _____
| |__     __ _    ___  |  _ \  |_ _| \ \   / / | ____|
| '_ \   / _` |  / __| | | | |  | |   \ \ / /  |  _|
| |_) | | (_| | | (__  | |_| |  | |    \ V /   | |___
|_.__/   \__,_|  \___| |____/  |___|    \_/    |_____|


analyses bacdive data for local analysis.
   ************************************************
   Gaurav Sablok
   Email: codeprog@icloud.com
   ************************************************

Usage: bacdive <COMMAND>

Commands:
 id
 species               please provide the species that need to be searched
 strain                please provide the category2 that you want to look,
 id-list               this will list all the available unique ids present in the bacdive
 species-list          this will list all the unique species present in the bacdive
 strainlist            this will list all the available countries in the bacdive
 id-list-analyze       present the list of the unique ids present
 species-list-analyze  provide the species present in the bacdive
 designation-list      provide the designation header present in the bacdive
 strain-number-list    provide the strain number present in the bacdive
 strainheader-list     provide the strain header present in the bacdive
 id-search             search for the specific id and json output
 species-search        search for the specific species and json output
 designation-search    search for the specific designation and json output
 strain-search         search for the specific strain and json output
 web-mine              get webmine results from the bacdive
 help                  Print this message or the help of the given subcommand(s)

Options:
 -h, --help     Print help
 -V, --version  Print version
 ```

- Example usage of the bacdive:

```
bacdive id ./sample-file/bacdive-2025-01-17.csv 159652
The ids are: [BacdiveSpeciesJson { id: "159652", species: "Abditibacterium utsteinense", strain: "DSM 105287", information: " LMG 29911,Top surface sample consisting of weathered granite parent material, elevation 1382 m,Antarctica,Australia and Oceania,Environmental,Terrestrial,Geologic" }, BacdiveSpeciesJson { id: "159652", species: "same species", strain: "same strain", information: ",,,,,Climate,Cold,Alpine" }]

bacdive species ./sample-file/bacdive-2025-01-17.csv Actinocatenispora-thailandica
The species and the associated information are: [BacdiveSpeciesJson { id: "7795", species: "Actinocatenispora thailandica", strain: "DSM 44816", information: " JCM 12343, PCU 235, BCRC 16831, CGMCC 4.5560, CIP 109347, NBRC 105041, NCIMB 14320,Environment, Soil, peat swamp forrestPeat swamp forest soilsoil,Thailand,Asia,Environmental,Terrestrial,Soil" }, BacdiveSpeciesJson { id: "161217", species: "Actinocatenispora thailandica", strain: "JCM 12344", information: " PCU 236,Peat swamp forest soil,Thailand,Asia,,," }]

bacdive strain ./sample-file/bacdive-2025-01-17.csv DSM17304
The strain specific information are as follows:[BacdiveSpeciesJson { id: "268", species: "Aeromonas tecta", strain: "DSM 17304", information: "tap water,Switzerland,Europe,Engineered,Built environment," }]
```

```
bacdive id-list ./sample-file/bacdive-2025-01-17.csv
bacdive species-list ./sample-file/bacdive-2025-01-17.csv
bacdive id-list-analyze ./sample-file/bacdive-2025-01-17.csv
bacdive species-list-analyze ./sample-file/advsearch_bacdive_2025-01-20.csv
bacdive designation-list ./sample-file/advsearch_bacdive_2025-01-20.csv
bacdive strain-number-list ./sample-file/advsearch_bacdive_2025-01-20.csv
bacdive strainheader-list ./sample-file/advsearch_bacdive_2025-01-20.csv
```

- Web miner

```
bacdive web-mine 159652
Strain identifier
BacDive ID: -159652
Type strain:
Species: -Abditibacterium utsteinense
Strain Designation: -R-68213
Culture col. no.: -DSM 105287-, -LMG 29911
Strain history: -<- A. Willems, Lab. Microbiology, Ghent Univ., Belgium <- G. Tahon, Laboratory of Microbiology, Ghent University, Gent, Belgium
NCBI tax ID(s): -1960156 (species)
The command has finished:The webmine results are as follows
```

Gaurav Sablok
