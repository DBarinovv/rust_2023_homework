# Course syllabus

This document is the course syllabus. It contains the grading and lecture course program.

## Grading

- You have to solve all of the obligatory tasks to achieve a Satisfactory mark. Moreover, it's enough for a Satisfactory mark.
- Every task has a score X. After the deadline (usually 3 weeks) the score linearly lowers from X to 0.75X (can be changed later).
- There are 3 projects in the semester. Every project costs 1 point. After the project deadline (usually 4 weeks) the score linearly lowers from 1 to 1/2 for 2 weeks. Your maximum mark is determined by _ceil_ value of these points:
  - 0 points - Satisfactory(4) is maximum;
  - 1 point - Good(7) is maximum;
  - 2 points - Excellent(9) is maximum;
  - 3 points - Excellent(10) is maximum.
- Let `project_max()` be the function, which returns your maximum mark (4, 7, 9 or 10) depending on project points, `my_score()` the sum of your scores, and `max_score()` the sum of all scores without deadline penalties. If all of the obligatory problems are solved, then your mark is:
  - `min(project_max(), max(3, round(10 * my_score() / max_score())))`

## Lectures program

00. Introduction to the course
    - Motivation
    - Basic syntax
    - Primitive types

01. Basics
    - Structures and enums (also with generics)
    - `if`, `while`, `for`, `loop`
    - Match
    - Borrow checker
    - Option and Result

02. Traits
    - Traits syntax
    - `dyn`, `impl` keywords
    - Some standard traits (`Copy`, `Clone`)
    
03. More Traits
    - More traits
    - Some std containers
    - Operators overloading using traits

04. Rewind
    - `String` and `&str`
    - `Box` and `Rc`
    - `Dyn` keyword

05. Cell and RefCell
    - `Cell` implementation
    - `RefCell` implementation
    - `Rc` 

06. Cargo and Modules
    - Cargo versions and tools
    - Modules and their visability
    - `use`, `mod`, `super`, `crate` keywords

07. CLosures
    - Closure types
    - `Fn`, `FnMut`, `FnOnce`
    - Function pointers and items

08. Lifetimes
    - Definition
    - Example for structures, functions, traits, etc.
    - Elision rules

09. Channels
    - Definition 
    - Basic implementation (using `Mutex` and `Condvar`)
    - Flavours

### [Additional reading](reading-list.md)
