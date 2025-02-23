# bacdive-analyzer
 - rust driven bacdive analyzer. 
 - analyses all the bacdive data and the bacdive function download the coresponding bacdive pages for the offline viewing. 
 - id, isolation, strain, type_strain download the corresponding webpages from the bacdive. 
 - please see the last commit message and if it says compiled binary then it is completed or else still in development version.

 ```
 cargo build 
 ```
 
 ```
 ➜  bacdive-analyzer git:(main) ✗ ./target/debug/bacdive-analyzer -h
 Usage: bacdive-analyzer <COMMAND>

 Commands:
  id-list             present the list of the unique ids present
  species-list        provide the species present in the bacdive
  designation-list    provide the designation header present in the bacdive
  strain-number-list  provide the strain number present in the bacdive
  strainheader-list   provide the strain header present in the bacdive
  id-search           search for the specific id and json output
  species-search      search for the specific species and json output
  designation-search  search for the specific designation and json output
  strain-search       search for the specific strain and json output
  help                Print this message or the help of the given subcommand(s)

 Options:
  -h, --help     Print help
  -V, --version  Print version
 ```
 ```
 ➜  bacdive-analyzer git:(main) ✗ ./target/debug/bacdive-analyzer id-search -h
 search for the specific id and json output

 Usage: bacdive-analyzer id-search <BACDIVE_ANALYZER> [ID]

 Arguments:
  <BACDIVE_ANALYZER>  please provide the path to the bacdive file
  [ID]                please provide the specific id that you want to look

 Options:
  -h, --help  Print help
 ➜  bacdive-analyzer git:(main) ✗ ./target/debug/bacdive-analyzer id-search \ 
                         ./sample-files/advsearch_bacdive_2025-01-20.csv \
                                             110104 \
 The id of the species is:"110104"
 The species number is "Streptomyces sp."
 The designation header is: "ST014628(HKI) BI 034610"
```
```
  ./target/debug/bacdive-analyzer species-search ./sample-files/advsearch_bacdive_2025-01-20.csv Streptomyces

 The id of the species is:"133052"
The species number is "Streptomyces zhihengii"
The designation header is: "DSM 42176 CGMCC 4.7248 KCTC 39115 YIM T1021"

The id of the species is:"133638"
The species number is "Streptomyces zinciresistens"
The designation header is: "K42ACCC 41871 HAMBI 3107 CCNWNQ 0016 JCM 180801"

The id of the species is:"170372"
The species number is "Streptomyces zingiberis"
The designation header is: "PLAI 1-29NBRC 113170 TBRC 76451"

The id of the species is:"133870"
The species number is "Streptomyces ziwulingensis"
The designation header is: "F22CCNWFX 0001 JCM 18081 ACCC 41875 DSM 421301"
```
```
➜  bacdive-analyzer git:(main) ✗ ./target/debug/bacdive-analyzer strain-search ./sample-files/advsearch_bacdive_2025-01-20.csv 18081
The id of the species is:"17628"
The species number is "Patulibacter minatonensis"
The designation header is: "KV-614DSM 18081 CIP 109166 JCM 12834 NBRC 100761 NRRL B-24346 KCTC 19436 NCIMB 143471"

The id of the species is:"24608"
The species number is "Streptococcus dentasini"
The designation header is: "DSM 25137 JCM 17943 NUM 18081"

The id of the species is:"133870"
The species number is "Streptomyces ziwulingensis"
The designation header is: "F22CCNWFX 0001 JCM 18081 ACCC 41875 DSM 421301"
```

 Gaurav Sablok
