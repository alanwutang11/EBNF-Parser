

## EBNF Parser

This project takes a ".x" file, and if successfully parsed based on the given project EBNF rules, outputs a colored "out.xhtml" file.

## Running the program: 

To run the program, run this command in the project directory: 

$ cargo run [FILE PATH]

For example: 

$ cargo run ./src/example1.x

the expected output for this example will closely match the example1.xhtml file (spacing is the only thing that is slightly different)

Output: 
 
    1) To show functionality of the scanner, a print for each token with the given text and token type are printed to the terminal

    2) If there is a successful parse of the input file, an output file called "out.xhtml" will be generated in the project folder. This file is 
    a colored version of the input file based on format.csv

    3) If there is an error anywhere in the parsing, the program will panic with an error message


Project notes: 

-Scanner should work fine 

-most (not all) parsing errors should be caught from the input file

-for each error in the input file, the corresponding error should be correct most of the time. There are certainly some errors that will be generated, but won't display the 
correct corresponding message

-all identifiers were classified as token type: VARIABLE during the scanner phase. During parsing, identifiers would be classified as a token type: FUNCTION instead of 
VARIABLE if the token was indeed a function. 
