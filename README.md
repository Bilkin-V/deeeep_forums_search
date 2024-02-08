# Deep Forums Search 
> Don't worry about any programming terminology! There are clear instructions in the usage heading later!

Deep forums search is a basic command line tool for searching the forums. It can: 

- Search by username
- Search by title
- Search by category
- Custom timeout
- Custom search length

```
Usage: dbforums_search.exe [OPTIONS] [SEARCH]

Arguments:
  [SEARCH]  The searched for title

Options:
  -u, --user <USER>          Filters results by username
  -i, --id <ID>              Filters results by user id
      --category <CATEGORY>  Filters results by category
  -t, --timeout <TIMEOUT>    Sets a custom search length in seconds. Can be specified as a decimal, will be rounded to the nearest millisecond [default: 30]
  -c, --count <COUNT>        Number of posts to parse [default: 16000]
  -d, --display <DISPLAY>    Number of posts to display [default: 10]
  -h, --help                 Print help
  -V, --version              Print version
                                                      
```
