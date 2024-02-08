# Deep Forums Search 
> Don't worry about any programming terminology! There are clear instructions in the usage heading later!

Deep forums search is a basic command line tool for searching the forums. It can: 

- Search by username
- Search by title
- Search by category
- Custom timeout
- Custom search length

Flags list:

```
Usage: dfsearch.exe [OPTIONS] [SEARCH]

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

## Usage 
> Windows specific

In this website, navigate to "releases/windows". There should be a file called `dfsearch.exe`. Click and download it. 

Then, open the `Terminal` app via the windows search bar. Run the following command: 

```shell
cd "[path to dfsearch.exe]"
```

For example, if `dfsearch.exe` was located in my downloads, then I would run: 

```shell
cd "C:\Users\Bilkin\Downloads\"
```
Then, execute the command: 
```shell
.\dfsearch.exe "[some expression]"
```
Replace the value within `"[some expression]"` with the desired title. After 10 to 15 seconds, it should output ten or less results, ordered by relevance. For example, executing `.\dfsearch.exe "Question about bots` should give: 
```
TITLE: Question about bots
USER: Bilkin
URL: https://beta.deeeep.io/forum/en/34037
```
Copy and past the url or use `Ctrl + Click` to navigate to the forum post. 