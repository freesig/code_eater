# Reference

## Add a flag
All single source flags start with `\#S:FLAG,FLAG=param,FLAG`
## Include flag
`INCLUDE` All code blocks in your language proceeding this tag will be included in the generated _code_ file.
## Skip flag
`SKIP` All code blocks in your language proceeding this tag. No blocks will be included in the generated _code_ file until an `INCLUDE` flag is hit.
## External flag
`EXTERNAL=path/to/file` Include the code from this external file in the generated _code_ file at this location.
## Hide flag
`HIDE` Hide only the next code block from showing up in the generate _md_ file.
## Check flag
`CHECK=lang` is used to generate a block of code for the user to check their work against at that point in the tutorial.
## Extra flag
Sometimes you want to include an extra piece of information in check block. The extra flag includes the following code block in the next check block. It should be used right above a check.
## Mode flag
`MODE` is used to create different runs from the same language.
