# bvb2ynab

Bvb2ynab is a Rust command-line app for converting Bremische Volksbank CSV bank
statements into a form supported by YNAB import.

## Project status

Bvb2ynab is still in development, but it works well enough to be useful for me.

## Installation instructions

If you use Rust, you can build this by running following in the top level of the checked out repository 
```
$ cargo install --path . 
```

If you don't use Rust and just want the application to convert with,  let me know so I can help.

## Usage
As bvb2ynab tries to do one thing only.

As BVB still uses ISO-8859-15, initial converting is needed, and on Linux iconv does that really well.
Finally, redirect the output to a file in the end.

```
$ iconv -f iso-8859-15 -t utf-8 -o ~/out.csv ~/original.csv
$ bvb2ynab ~/out.csv >> final.csv 
```

If you are not a Linux user and all that feels a bit much for you, please let me know. The next version of 
bvb2ynab could very well do both of those steps, if there is a need for that.

Upon importing to YNAB, the recommendation is to uncheck importing the memos, unless 
you want a lot of bank details there.

## Contributing
Bug reports, feature requests, feedback and pull requests are welcome.
 
Please keep the tests and example data in example-file-in-utf8.csv up to date when you 
change or add code!


## License
[MIT](https://choosealicense.com/licenses/mit/)
