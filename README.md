# bvb2ynab

Bvb2ynab is a Rust command-line app for converting Bremische Volksbank CSV bank
statements into a form supported by YNAB import.

## Project status

Bvb2ynab is still in development, but it should be pretty harmless to try.

## Installation instructions
Don't ask me yet, I've been running this with cargo so far, as I'm new to Rust :)


## Usage
As bvb2ynab tries to do one thing only.

As BVB still uses ISO-8859-15, initial converting is needed, and iconv does that really well.
Finally, redirect the output to a file in the end.

```
$ iconv -f iso-8859-15 -t utf-8 -o ~/out.csv ~/original.csv
$ bvb2ynab ~/out.csv >> final.csv 

```

Upon importing to YNAB, the recommendation is to uncheck importing the memos, unless 
you want a lot of bank details there.

## Contributing
It's probably too early for anyone to do that, but pull requests are welcome. 
Please keep the tests up to date though.


## License
[GNU GPLv3](https://choosealicense.com/licenses/gpl-3.0/)
