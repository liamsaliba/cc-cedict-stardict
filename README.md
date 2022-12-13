# cedict

Takes [CC-CEDICT Chinese-English dictionary](https://www.mdbg.net/chinese/dictionary?page=cc-cedict), converts it to StarDict format.

Specifically for use with Koreader.

First time using Rust!

## Usage

Needs `cargo`.

```sh
cargo run

# cedict.csv created, turn it into stardict format

penelope -i cedict.csv -j csv --csv-fs "\t" --csv-ls "\n\n" -f cn -t en -p stardict -o cedict.zip --merge-definitions --merge-separator "\n" -d --title "CC-CEDICT 汉英词典"

unzip cedict.zip
```

Test that it worked:
```sh
sdcv --utf8-input --utf8-output --data-dir . -c -e 我
sdcv --utf8-input --utf8-output --data-dir . -c -e -n --json 跑步
```

Both of those should give correct definitions.

## Things to do later
- Have `cargo test` check that `sdcv` gives the correct output.
- HTML output
- Monthly releases? (via GitHub Actions)
