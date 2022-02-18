# The Golden

## Table of contents

1. [Basic info](#basic-info)
2. [How to run your code](#run-code)<br>
   2.1 [Arguments](#run-code-args)<br>
   2.2 [Flags](#run-code-flags)
3. [Main features](#main-features)
4. [Important notes](#important-notes)
5. [Mechanics](#mechanics)
6. [Syntax](#syntax)
7. [Incoming features](#incoming-features)
8. [Examples](#examples)

## Basic info <a name="basic-info"></a>

This language is an extension of a faily popular language called Brainfuck. It takes the beauty of not using any letters in the code from it but also provides some handy features, like printing output as numbers instead of characters and more.
<br>Its purpose is to let people make Brainfuck-styled programs less painfully.

## How to run your code <a name="run-code"></a>

All you need to do it run the interpreter file with Python 3.5 and higher.

### Arguments <a name="run-code-args"></a>

You can run the code with some arguments including:

-   The maumivu.au file location (you will be asked to provide it if you don't include it in the arguments)
-   Some flags

### Flags <a name="run-code-flags"></a>

See the table below for some flags you can provide when running your code.
| Flag | Usage | Effect |
|:-----|:------|:------|
| - \<code\> | `- '!!![~]:` | You can provide some code to be ran by the interpreter - no need to have a maumivu.au file |
| --debug | `--debug` | Enabled debug mode - print parsed commands, which command was ran and the memory state at the end of execution |
| --debug-heavy | `--debug-heavy` | Enabled heavy debug mode - print all the things printed in debug mode + stop for 0.5 seconds after each command and print the memory state |
| --disable-warnings | `--disable-warnings` | Disable all warnings |
| --disable-path-warning | `--disable-path-warning` | Disable the "No path provided" warning (fired when running code from args) |
| --disable-too-left-pointer-warning | `--disable-too-left-pointer-warning` | Disable the warning fired when you go to the -1 index in memory |

## Main features <a name="main-features"></a>

How good or bad the features of this language are is completely subjective, but here are some of them:

-   Brainfuck-like syntax - other will have no idea wth your code does
-   Easy operations chaining - forget code looking like `>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>.<<<<<<<<<<<<<<<<<<<<<<<<<<`, now you can do `|49|\,|26|<` to achieve the exact same result
-   Easy arithmetics - tired of multiplication in O(N^2) time? The solution is here! Just do `*` and you are done in ~O(1)
-   Decimal numbers - pretty self-explanatory, but now you can use decimal numbers in your code
-   And much more!

## Important notes <a name="important-notes"></a>

This language uses multiple memory rows - you have access to 2 global memory rows and 2 local memory rows. Both with global and local memory, you are always using just one row at a time. I will be referring to those rows as `active` (the one you are using) and `inactive` (the one you are not using).<br>
Unless said otherwise, `cell` is referring to the selected cell (and also if not said otherwise in the active global memory row).<br>

## Mechanics <a name="mechanics"></a>

The main file has to be named `maumivu.au`. This isn't required with command-line-provided code (obviously).
When converting numbers to characters and vice versa, the ASCII table is used.<br>
The memory has unlimited size and consists of double-precision numbers. When you go to an unexisting index (to the right) that cell is created with the value of 0.<br>
If you go into memory index -1, a 0 is added at that position and the whole memory is shifted one cell to the right. While this is allowed, I would discourage you from doing it since it can be fairly slow (compared to other operations). That's why it will fire a warning.<br>
Loops function the exact same way as in Brainfuck - they only run if the current cell value isn't 0. This language also offers do-while loops, which ignore the check the first time.<br>
You can chain commands by putting `||` in front of them. You can also put a number between those pipes. If you decide to put a number in there, the command right after it will run `floor(the number)` times. If you leave it empty, the code will run `floor(cell value)` times. If the value is negative, the opposite command will be ran (see the table below). If the value is 0, it won't be ran at all.<br>
| Command | Opposite command |
|:-----|:------|
| ! | ~ |
| ~ | ! |
| + | - |
| - | + |
| _ | / |
| / | _ |
| > | < |
| < | > |

## Syntax <a name="syntax"></a>

The magic of Brainfuck-like syntax is that it is easy and extremely difficult at the same time. Here are all the commands the interpreter will understand:
| Command | Explanation | Showcase | Chainable? | Usable on local memory? |
|:-----|:------|:------|:------|:------|
| ! | Adds one to the current cell | `!` | Yes | Yes |
| ~ | Subtracts one from the current cell | `~` | Yes | Yes |
| + | Adds the cell in the inactive row to the cell in the active row | `+` | Yes | Yes |
| - | Subtracts the cell in the inactive row from the cell in the active row | `-` | Yes | Yes |
| * | Multiplies the cell in the active row by the cell in the inactive row | `*`| Yes | Yes |
| / | Divides the cell in the active row by the cell in the inactive row |`/`| Yes | Yes |
| _ | Floors the current cell value (towards -infinity) |`_`| No | Yes |
| & | Ceils the current cell value (towards +infinity) |`&`| No | Yes |
| ` | Sets the cell to a random number from 0 (inclusive) to 1 (exclusive) | <code>`</code> | No | Yes |
| > | Move the cell pointer one to the right | `>`| Yes | Yes |
| < | Move the cell pointer one to the left |`<`| Yes | Yes |
| ^ | Switch active memory (sets the active as inactive and the inactive as active) |`^`| No | Yes |
| $. | Sets the cell to the value of user input as a number (if they input 69, the cell value will be 69) |`$.` | No | Yes |
| $, | Sets the cell to the value of user input as a character (if they input E, the cell value will be 69) | `$,`| No | Yes |
| \. | Output the cell as a number (if the cell value is 69, 69 will be printed) |`\.`| No | Yes |
| \, | Output the cell as a character (if the cell value is 69, E will be printed) |`\,`| No | Yes |
| [ | Start a while loop |`[` | No | Yes |
| ] | End a while loop | `]` | No | Yes |
| [@ | Start a do-while loop | `[@` | No | Yes |
| @] | End a do-while loop | `@]` | No | Yes |
| ?? | Sets the cell value to its index | `??` | No | Yes |
| ?= | If the cells in the active and inactive rows have the same value, break the loop | `[?=]` | Yes | Yes |
| ?< | If the cell in the active row has a lower value than the cell in the inactive row, break the loop | `[?<]` | Yes | Yes |
| ?> | If the cell in the active row has a higher value than the cell in the inactive row, break the loop | `[?>]` | Yes | Yes |
| ; | Switch the values of the active global cell and the active local cell | `;` | No | Yes |
| "" | Comments | `"This is a comment"` | No | No |

Each line has to end with a punctuation (`:`) or else the program will crash.

## Incoming features <a name="incoming-features"></a>

-   Functions
-   Running other files
-   A compiled interpreter

## Examples <a name="examples"></a>

Here are some examples written in this language:<br>
"Hello, world!":

```
|72|!\,|29|!\,|7|!\,\,|3|!\,|67|~\,|12|~\,|87|!\,|8|~\,|3|!\,|6|~\,|8|~\,|67|~\,:
```

Fibonacci sequence:

```
!>|10|!<^>|10|!<[@^+\.>\,<@]:
```

You can find all the examples in the [examples folder](https://github.com/Pandicon/The-Golden/tree/main/examples).
