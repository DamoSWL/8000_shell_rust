# 8000_shell_rust
# Project

Write a Shell in Rust.

## Objective:

It’s easy to view yourself as “not a real programmer.” There
are programs out there that everyone uses, and it’s easy to
put their developers on a pedestal. Although developing large
software projects isn’t easy, many times the basic idea of
that software is quite simple. Implementing it yourself is a
fun way to show that you have what it takes to be a real
programmer.

## Description:
A shell does three main things in its lifetime.  
(1)  Initialize  
(2)  Interpret  
(3)  Terminate  

‘~’, `..`, ‘/’ are valid (along with alphanumeric characters) and stand for traditional meaning.  
pwd:
```bash
pwd [OPTION]
```
ls:
```bash
ls [OPTION]... [FILE]...
```
Option: -l, -r, -s, --file-type (match file extension).  
Exit:
```bash
exit
```

## Programming Language used
Rust

## Contributors
Weili, Shi  
Sixiang, Zhang

## Releases:
0.1: initialization, pwd, exit

0.2: ls,cd

0.3: pipe (|), service (&), history

## How to run
To find the executable file
```bash
.target/debug/shell_rust
```



