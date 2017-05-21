# Glossy

Different Industries and professions often make heavy use of jargon and acronyms. This can make life difficult for people starting work in a new area. When faced with an unfamiliar term, we could search online. However, this requires opening a new tab and searching through a large list of possibly irrelevant results. Moreover, if the term you are searching for is company-specific, an online search may be futile. A company may provide an internal site with descriptions for jargon and acronyms. While useful, this still requires navigating to the browser and knowing which page contains the information you are after. These steps can be slow and painful.

As developers/engineers, I suggest a command line tool is more useful since our work is often heavily terminal-based. Jargon/acronyms with their descriptions can be provided in a separate file, preferably under version control. When confronted with an unfamiliar term, a query is as simple as `glo wtf`.

### Installation

To install Rust see - https://www.rust-lang.org/en-US/install.html

```
git clone https://github.com/Michaelt293/glossy.git

cd glossy

cargo install
```

### Configuration

To use glossy, a CSV file of terms (jargon/acronyms) and descriptions is required. (Where the first column is for terms and the second column is for descriptions.) The CSV file may or may not have headers.

The path for the CSV file must be exported using either the `GLOSSY_FILEPATH_HEADERS` or `GLOSSY_FILEPATH_NO_HEADERS` environmental variables.

### Example/Usage

*CSV file - severe-weather-parameters.csv*
```
parameter,description
LCL,lifting condensation level
LFC,level of free convection
EL,equilibrium level
CAPE,Convective Available Potential Energy
LI,Lifted Index
CIN,convective inhibition
DCAPE,Downdraft CAPE
SRH,Storm-Relative Helicity
```
source: http://www.spc.noaa.gov/sfctest/help/sfcoa.html

Export the required environmental variable (add the line below to your `.bash_profile` file for OSX).
```
export GLOSSY_FILEPATH_HEADERS=path-to-csv-file/severe-weather-parameters.csv
```

By default, the search is case insensitive.
```
$ glo cape
CAPE: Convective Available Potential Energy
```

We can make the search case sensitive using the -s flag.
```
$ glo -s LI
LI: Lifted Index
```
