# Rust text parser commandline tool example

A very simple Rust CSV parser commandline tool.

It expects data files like the below, with `###` comments at the end of the line, if desired.

```
FNAME,LNAME,AGE,ADDRESS,CITY,STATE,COUNTRY
tim, jones, 77,  123 Lift Street, Bulbury, Cornwall, UK
jim,patron,29,8 Infinity Lane,Vancounver,BC,Canada        ### Add a cool and important comment here!
alfred,haroldson,,433 Simpleton Way,New York,NY,USA
```

Consider adjusting the record struct to make your intended data.

### Getting started

The fastest way to get started is as such:

```
make && ./target/release/rust-text-parser --input example.csv
```

This will then print records and mention the number of broken lines:

```
Number of broken lines that were skipped: 2
Records:
[
    Record {
        FNAME: "tim",
        LNAME: "jones",
        AGE: Some(
            77,
        ),
        ADDRESS: "123 Lift Street",
        CITY: "Bulbury",
        STATE: "Cornwall",
        COUNTRY: "UK",
    },
]
```