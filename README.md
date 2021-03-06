# City Generator
I made this tool for the purposes of tabletop RPGs, trying to generate natural-sounding city names based on common prefixes and suffixes.

The idea first came about when Benjamin 'Yahtzee' Croshaw discussed his game "The Consuming Shadow", 
and mentioned that all the towns you visit in-game aren't necessarily real, 
but rather generated by combining common prefixes and suffixes of British towns, making plausible town names.

This tool works fairly simply: you feed it a JSON file containing a list of prefixes and suffixes, and it spits out a random city name.

The JSON file should look something like this:
```json
{
  "prefixes": [ "prefixes", "go", "here" ],
  "suffixes": [ "suffixes", "go", "here" ]
}
```

You generate the city by writing `city_generator list.json` where "list" is whatever you decided to call the file.
