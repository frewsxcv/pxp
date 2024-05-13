# Doc Strings

This is just a document to write down notes about how we parse and tokenise nowdoc and heredoc strings.

## Heredocs vs Nowdocs

Heredocs are basically just double-quoted strings. That means you can use interpolation, escape sequences, etc.

Nowdocs are single-quoted strings. There's no interpolation and no escape sequences. This means they're significantly easier to tokenise and parse, since you basically just look for everything between two matching identifiers.

If there is no matching label at the end, you just continue parsing everything as a string until the end of the document.

## Tokenisation

The first indicator for a heredoc or nowdoc is the `<<<` token. This marks the start of the string. Heredocs are then followed by a valid label, e.g. `EOF`, `TXT`, `HTML`. Nowdocs are the same, but the label is wrapped in single quotes. Then a line break is required. 

```
<<< ('?) <label> ('?) \n
```

