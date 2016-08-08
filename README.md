# termc

## Introduction
termc is a calculator for the command line.
It supports the basic operations ("+", "-", "*", "/" and "^") as well as the following mathematical functions:
- cos
- sin
- tan
- cot
- acos
- asin
- atan
- acot
- cosh
- sinh
- tanh
- ln
- exp
- sqrt
- pow (e.g. "pow(5, 2)" = 25)
- root (e.g. "root(4, 2)" = 2)

Futhermore, the following constants are supported:
- e
- pi
- i (the imaginary unit)

## Modes of Operation
termc supports two different modes of operation.

### Call mode
In this mode, the user can pass mathematical expressions as command line arguments to termc.
```sh
$ termc 1+2 5*7 "cos(pi)"
3;35;-1
```

### Interactive mode
For this mode, no additional command line arguments are passed to the call of termc.
It will then start the interactive mode.
```sh
$ termc
>>> 1+2
ans: 3

>>> 5*7
ans: 35
...
```

## License
[GNU GENERAL PUBLIC LICENSE Version 3, 29 June 2007](https://www.gnu.org/licenses/gpl.html)
A copy of the license can be found in the root directory of this repository.
