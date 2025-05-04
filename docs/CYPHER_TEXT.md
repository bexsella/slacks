# Cypher Text
Details for operators on the communication over the clacks network.

## Notes on the Clacks

The 8 shutters on a standard clacks tower are numbered in the following
sequence:

| Left | Right |
|:--: |:--: |
|  1  |  5  |
|  2  |  6  |
|  3  |  7  |
|  4  |  8  |

For the rest of this operators manual, when referring to shutter numbers we know
that when referring to shutter 1, we are referring to the top left shutter, and 
shutter 8 is the bottom right shutter.

This document uses the following symbols for representing a shutter being open
or closed:

| Open | Closed |
| :--: |  :--:  |
|   ■  |   -    |

## Numeric

Listed below are the Slacks semaphore representations of numbers.

### 1

| Left | Right |
|:--: |:--: |
|  ■  |  -  |
|  -  |  -  |
|  -  |  -  |
|  -  |  -  |

* Hexcode representation: `h01`
* Binary representation: `b0000_0001`

### 2

| Left | Right |
|:--: |:--: |
|  -  |  -  |
|  ■  |  -  |
|  -  |  -  |
|  -  |  -  |

* Hexcode representation: `h02`
* Binary representation: `b0000_0010`

### 3

| Left | Right |
|:--: |:--: |
|  -  |  -  |
|  -  |  -  |
|  ■  |  -  |
|  -  |  -  |

* Hexcode representation: `h04`
* Binary representation: `b0000_0100`

### 4

| Left | Right |
|:--: |:--: |
|  -  |  -  |
|  -  |  -  |
|  -  |  -  |
|  ■  |  -  |

* Hexcode representation: `h08`
* Binary representation: `b0000_1000`

### 5

| Left | Right |
|:--: |:--: |
|  -  |  ■  |
|  -  |  -  |
|  -  |  -  |
|  -  |  -  |

* Hexcode representation: `h10`
* Binary representation: `b0001_0000`

### 6

| Left | Right |
|:--: |:--: |
|  -  |  -  |
|  -  |  ■  |
|  -  |  -  |
|  -  |  -  |

* Hexcode representation: `h20`
* Binary representation: `b0010_0000`

### 7

| Left | Right |
|:--: |:--: |
|  -  |  -  |
|  -  |  -  |
|  -  |  ■  |
|  -  |  -  |

* Hexcode representation: `h40`
* Binary representation: `b0100_0000`

### 8

| Left | Right |
|:--: |:--: |
|  -  |  -  |
|  -  |  -  |
|  -  |  -  |
|  -  |  ■  |

* Hexcode representation: `h80`
* Binary representation: `b1000_0000`

### 9

| Left | Right |
|:--: |:--: |
|  ■  |  -  |
|  -  |  -  |
|  -  |  -  |
|  -  |  ■  |

* Hexcode representation: `h81`
* Binary representation: `b1000_0001`

### 0

| Left | Right |
|:--: |:--: |
|  -  |  -  |
|  ■  |  -  |
|  -  |  -  |
|  -  |  ■  |

* Hexcode representation: `h82`
* Binary representation: `b1000_0010`

## Alphabet

Listed below are the Slacks semaphore representations of Latin characters.

### A

| Left | Right |
|:--: |:--: |
|  ■  |  -  |
|  ■  |  -  |
|  -  |  -  |
|  -  |  -  |

* Hexcode representation: `h03`
* Binary representation: `b0000_0011`


### B

| Left | Right |
|:--: |:--: |
|  ■  |  -  |
|  ■  |  ■  |
|  -  |  -  |
|  -  |  -  |

* Hexcode representation: `h23`
* Binary representation: `b0010_0011`


### C

| Left | Right |
|:--: |:--: |
|  ■  |  ■  |
|  ■  |  ■  |
|  -  |  -  |
|  -  |  -  |

* Hexcode representation: `h33`
* Binary representation: `b0011_0011`


### D

| Left | Right |
|:--: |:--: |
|  -  |  ■  |
|  ■  |  ■  |
|  -  |  -  |
|  -  |  -  |

* Hexcode representation: `h32`
* Binary representation: `b0011_0010`


### E

| Left | Right |
|:--: |:--: |
|  -  |  ■  |
|  ■  |  -  |
|  -  |  ■  |
|  -  |  -  |

* Hexcode representation: `h52`
* Binary representation: `b1010_0010`


### F

| Left | Right |
|:--: |:--: |
|  ■  |  ■  |
|  ■  |  -  |
|  ■  |  -  |
|  -  |  -  |

* Hexcode representation: `h17`
* Binary representation: `b0001_0111`


### G

| Left | Right |
|:--: |:--: |
|  ■  |  -  |
|  ■  |  ■  |
|  -  |  ■  |
|  -  |  ■  |

* Hexcode representation: `h73`
* Binary representation: `b0111_0011`


### H

| Left | Right |
|:--: |:--: |
|  ■  |  ■  |
|  ■  |  ■  |
|  ■  |  ■  |
|  -  |  -  |

* Hexcode representation: `h77`
* Binary representation: `b0111_0111`


### I

| Left | Right |
|:--: |:--: |
|  ■  |  -  |
|  -  |  ■  |
|  -  |  ■  |
|  -  |  ■  |

* Hexcode representation: `hE1`
* Binary representation: `b1110_0001`


### J

| Left | Right |
|:--: |:--: |
|  -  |  ■  |
|  ■  |  -  |
|  ■  |  -  |
|  ■  |  -  |

* Hexcode representation: `h78`
* Binary representation: `b0111_1000`


### K

| Left | Right |
|:--: |:--: |
|  -  |  ■  |
|  ■  |  -  |
|  ■  |  -  |
|  -  |  ■  |

* Hexcode representation: `h96`
* Binary representation: `b1001_0110`


### L

| Left | Right |
|:--: |:--: |
|  -  |  -  |
|  ■  |  -  |
|  ■  |  -  |
|  ■  |  ■  |

* Hexcode representation: `h8E`
* Binary representation: `b1000_1110`


### M

| Left | Right |
|:--: |:--: |
|  ■  |  ■  |
|  -  |  -  |
|  ■  |  ■  |
|  -  |  -  |

* Hexcode representation: `h55`
* Binary representation: `b0101_0101`


### N

| Left | Right |
|:--: |:--: |
|  -  |  -  |
|  ■  |  ■  |
|  -  |  -  |
|  ■  |  ■  |

* Hexcode representation: `hAA`
* Binary representation: `b1010_1010`


### O

| Left | Right |
|:--: |:--: |
|  -  |  -  |
|  -  |  -  |
|  ■  |  ■  |
|  ■  |  ■  |

* Hexcode representation: `hCC`
* Binary representation: `b1100_1100`


### P

| Left | Right |
|:--: |:--: |
|  -  |  -  |
|  ■  |  ■  |
|  ■  |  -  |
|  ■  |  -  |

* Hexcode representation: `h2E`
* Binary representation: `b0010_1110`


### Q

| Left | Right |
|:--: |:--: |
|  -  |  -  |
|  ■  |  ■  |
|  -  |  ■  |
|  -  |  ■  |

* Hexcode representation: `hE2`
* Binary representation: `b1110_0100`


### R

| Left | Right |
|:--: |:--: |
|  -  |  ■  |
|  ■  |  -  |
|  -  |  ■  |
|  -  |  ■  |

* Hexcode representation: `hD2`
* Binary representation: `b1101_0010`


### S

| Left | Right |
|:--: |:--: |
|  -  |  ■  |
|  ■  |  -  |
|  -  |  ■  |
|  ■  |  -  |

* Hexcode representation: `h5A`
* Binary representation: `b0101_1010`


### T

| Left | Right |
|:--: |:--: |
|  ■  |  ■  |
|  ■  |  -  |
|  ■  |  -  |
|  ■  |  -  |

* Hexcode representation: `h1F`
* Binary representation: `b0001_1111`


### U

| Left | Right |
|:--: |:--: |
|  ■  |  ■  |
|  ■  |  -  |
|  ■  |  -  |
|  ■  |  ■  |

* Hexcode representation: `h9F`
* Binary representation: `b1001_1111`


### V

| Left | Right |
|:--: |:--: |
|  ■  |  ■  |
|  ■  |  -  |
|  ■  |  ■  |
|  -  |  -  |

* Hexcode representation: `h57`
* Binary representation: `b0101_0111`


### W

| Left | Right |
|:--: |:--: |
|  ■  |  ■  |
|  ■  |  -  |
|  ■  |  ■  |
|  ■  |  ■  |

* Hexcode representation: `hDF`
* Binary representation: `b1101_1111`


### X

| Left | Right |
|:--: |:--: |
|  -  |  -  |
|  ■  |  ■  |
|  ■  |  ■  |
|  -  |  -  |

* Hexcode representation: `h66`
* Binary representation: `b0110_0110`


### Y

| Left | Right |
|:--: |:--: |
|  ■  |  -  |
|  -  |  ■  |
|  ■  |  -  |
|  ■  |  -  |

* Hexcode representation: `h2D`
* Binary representation: `b0010_1101`


### Z

| Left | Right |
|:--: |:--: |
|  ■  |  -  |
|  -  |  ■  |
|  ■  |  -  |
|  -  |  ■  |

* Hexcode representation: `hA5`
* Binary representation: `b1010_0101`

## Punctuation/Control Characters

### Space

| Left | Right |
|:----:|:-----:|
|  -   |   -   |
|  -   |   -   |
|  -   |   -   |
|  ■   |   ■   |

* Hexcode representation: `h88`
* Binary representation: `b1000_1000`

### Message Start

| Left | Right |
|:----:|:-----:|
|  -   |   -   |
|  ■   |   -   |
|  ■   |   ■   |
|  ■   |   -   |

* Hexcode representation: `h4E`
* Binary representation: `b0100_1110`

### Message End

| Left | Right |
|:----:|:-----:|
|  -   |   -   |
|  -   |   ■   |
|  ■   |   ■   |
|  -   |   ■   |

* Hexcode representation: `hE4`
* Binary representation: `b1110_0100`

### Error Code

| Left | Right |
|:----:|:-----:|
|  ■   |   ■   |
|  ■   |   ■   |
|  ■   |   ■   |
|  ■   |   ■   |

* Hexcode representation: `hff`
* Binary representation: `b1111_1111`

## Initial Test Setup

To ensure the clacks tower is fully operational it is beneficial to run through
the complete sequence of the aforementioned alphanumeric sequence and ensure
that all shutters are operating correctly and as expected.

## Tower Codes

| Character | Definition                               |
|:---------:|:-----------------------------------------|
|     G     | Send message on                          |
|     N     | Do not log                               |
|     U     | Turn around at the end, and send it back |
|     M     | Message start                            |

## Clacks Message Format

A clacks message will always start with an M character followed by 4 characters
representing the address code of the tower to that the message is addressed
to, and the 4 characters representing the address code of the tower that originally
sent the message. Any associated codes that may be required for the message will
be sent immediately after the address codes. The content of the message will start
with the Message Start code, and once the message has been communicated, the
Message End code will be sent.

### Example Message

#### String Formatting:
`Ma1febeed[hello world]`
