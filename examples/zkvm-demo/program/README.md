# Regex Checker

This code inputs a regular expression and a string and outputs whether the string fits the pattern.


## Making Modifications

In order to modify the zkVM program, you should clone this circuit and develop within `program/src/main.rs`. 
Recompiling the ELF in order to deploy an updated build to Sindri can be accomplished via:
```
cd program
cargo prove build --docker --tag v4.0.0 --output-directory .
```

You should make sure that the tag you use is compatible with the Sp1 version in the `Cargo.toml` in the program directory and the Sp1 version indicated in your Sindri manifest (`sindri.json` in the top level of your project).
