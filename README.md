# termc

## Introduction and Goals
**termc** is a calculator for the command line.
The goal of this project is to provide an *easy-to-use and intuitive* command line calculator with a basic range of functions.
It supports the basic operations ("+", "-", "*", "/" and "^") as well as the following built-in mathematical functions:
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
- coth
- acosh
- asinh
- atanh
- acoth
- ln
- exp
- sqrt
- pow (e.g. "pow(5, 2)" = 25)
- root (e.g. "root(4, 2)" = 2)

Futhermore, the following built-in constants are supported:
- e
- pi
- i (the imaginary unit)

## Specialities
### Complex numbers
**termc** supports complex numbers.
Example:
```sh
$ termc
>>> sin(5+3i)
ans = -9.654125476854839+2.841692295606352i
```

### User-defined constants
**termc** supports the definition of custom constants.
Example:
```sh
$ termc
>>> custom_constant = 5*pi/4

>>> cos(custom_constant)
ans = -0.7071067811865477
```

### User-defined functions
**termc** supports the definition of custom functions.
Example:
```sh
$ termc
>>> f(a, b, c) = a + b - c

>>> f(5, 3-2i, sin(pi/2))
ans = 7-2i
```

### Serialization and Deserialization to / from JSON
**termc** supports the serialization and deserialization of all custom functions and constants.
Therefore, all definitions can be saved to a file.
Example:
```sh
$ termc
>>> f(x) = x^2

>>> c = 79.882

>>> save /home/kantic/termc.save

>>> exit

$ termc
>>> load /home/kantic/termc.save

>>> f(c)
ans = 6381.133924000001

>>> ...
```

### Command History
**termc** remembers the user inputs in a session. Thus, the user is able to quickly get previous inputs by using the
up and down arrow-keys.

### Guiding error messages
**termc** prints helpful error messages if the user made some mistakes in his input.
Example:
```sh
$ termc
>>> pow(2.7, 3
Error: Expected symbol ")".
pow(2.7, 3
          ^~~~

>>> cis(pi)
Error: Expected function or operation.
cis(pi)
  ^~~~ Found: cis
```

### MultiOS
**termc** compilation has been tested on both linux (Debian 8) and Windows (Windows 10).
All unix-like operating systems on which rust is available should work, too!

## Modes of Operation
**termc** supports two different modes of operation.

### Call mode
In this mode, the user can pass mathematical expressions as command line arguments to termc.
```sh
$ termc 1+2 5*7 "cos(pi)"
3;35;-1
```

### Interactive mode
For this mode, no additional command line arguments are passed to the call of **termc**.
It will then start the interactive mode.
```sh
$ termc
>>> 1+2
ans = 3

>>> 5*7
ans = 35
...
```

## License
[GNU GENERAL PUBLIC LICENSE Version 3, 29 June 2007](https://www.gnu.org/licenses/gpl.html)
A copy of the license can be found in the root directory of this repository.

