# wordlister

Name and surname wordlist generator.

Once the name and surname combined list is created, you can use [namemash](https://gist.github.com/superkojiman/11076951) to create a list of possible usernames. 

### Build with

```
cargo build --release
```

### run with

```
cargo run --release names.txt surnames.txt
```

### Example lists

surnames: https://github.com/danielmiessler/SecLists/blob/master/Miscellaneous/security-question-answers/common-surnames.txt

names: https://github.com/dominictarr/random-name/blob/master/first-names.txt
