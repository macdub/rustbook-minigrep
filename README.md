# *minigrep* - Rust Book Project Chapter 12
This project is to review and implement the techniques and lessons learned from chapters 1-11.  
*minigrep* is a simple replication of the *grep* command line tool.  

## Project Description
1. Program accepts command line arguments in this form: `$ minigrep <QUERY> <FILE>`
2. Returns the matched lines from the file:  
```bash
$ minigrep frog poem.txt
Searching for 'frog'
In file 'poem.txt'
How public, like a frog
```
3. Should throw an error when there aren't enough arguments passed in
```bash
$ minigrep frog
Problem parsing arguments: not enough arguments
```