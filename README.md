# wisc
---
# About

The WISC language is a unique language that can be compiled or interpreted, weakly typed, or strongly typed, depending on the developers preference. It is perfectly suited for cross-platform CICD, Development, Web Servers, CLI Tooling, and much more.
This project has goals that will ensure success and adoption of the language. The goals are...

- **Tooling is first class**. The language server, the cli, the parser, everything for developers to be successful will all be included in decisions and apart of the main project.
- **Make the fastest interpreted language available**. Tools will be able to output optimal performance suggestions for interpreted scripts. The language is designed around executing scripts as fast as possible.
- **Provide the best embedded support**. Embedded systems require demanding performance use of their hardware. Supporting embedded systems will force the tooling, to be as efficient as possible. Thus, further helping the second goal.
- **Documentation is key**. Making documentation another high priority, will ensure that any newcomers to the language get the most up to date features,knowledge, and tutorials for easy use. After standardization we will provide upgrade paths for everyone.
- **Make it easy to use**. The first class tooling, the documentation, and clear goals will make this tool and language easy to use.

---
What does wisc stand for?

- **W**eakly (typed)
- **I**nterpreted
- **S**trongly (typed) 
- **C**ompiled language.

Binaries are not cross-platform, Scripts are cross-platform. Developers will be able to create minified scripts that can be ran on any machine, compiled binaries, or both. Many developers work on different systems, and the tools output a binary that is specific to x86-64, riscv32, arm32, the list goes on.
In the business world for CICD, many developers would work on windows, but then pipelines or servers could be linux based, or if the company is fancy, they might even have their developers be on mac. We need to make the code that is developed, portable. This is why in some situations, the scripts themselves need to be cross-platform.

There are two main factions in the software building community. 

One side cares strictly about the correctness of the code, and has no issues enforcing the strictest type checking. More time is spent receiving feedback from the compiler. This reduces the feedback of runtime behavior, but ensures correctness, and often, fewer bugs.

While the other side, has no problems with implicit casts, doesn't type check and allows you to run code. When working with javascript, I find the lack of typing nice at times when I just want to see the code run, inspect breakpoints, variables in memory, etc. Actually running the code, and seeing the actual behavior has benefits. 

Wisc has the most powerful type handling, and allows much more granular control over static analysis.

---
# Contents
- [About](#about)
- [Introduction](#introduction)
- [Installation](#installation)
- [Beginners](#beginners)
- [Language](#language)
	- [Program and Scope](#program-and-scope)
	- [Types](#types)
	- [Properties](#properties)
	- [Variables](#variables)
	- [Functions](#functions)
	- [Identifiers](#identifiers)
	- [Builtins](#builtins)
	- [Enums](#enums)
	- [Interfaces and Generics](#interfaces-and-generics)
	- [Yielded Types](#enums)
	- [Assembly](#assembly)
	- [Types List](#types-list)
	- [Builtins List](#builtins-list)
	- [Valid Syntax](#valid-syntax)
- [Tutorials](#tutorials)
	- [My First Project](#my-first-project)
  - [Interfaces](#interfaces-tutorial)
- [Default Behavior](#default-behavior)
- [Projects](#projects)
- [Embedded](#embedded)
	- [Script Based](#script-based)
	- [Linux Based](#linux-based)
	- [Freestanding](#freestanding)
- [Minifier](#minifier)
- [Linter](#linter)
- [Compiler](#compiler)
- [Library](#library)

---
# Introduction


---
# Installation 

---
# Beginners

---
# Language

#### Program and Scope
A valid program is a program which syntax for which the entire file, can be parsed by the `runner`. This is why a program can be valid, but have several bugs, or undefined behavior.

Every open curly brace `{` must have a closing brace `}`.
This will be referred to as a scope. Every `{}` is a scope. A file is an implicit scope.
Every scope must have an expression. This can be achieved many ways.

- A declaration `@`
- A builtin `%`
- Another scope `{}`
- Evaluation of a valid expression

examples of valid programs. Each line a separate program
```
{} // evaluates to null
{{}} // null evaluates to null
{5 + 5} // evaluates to 10
{"valid"} // evaluates to "valid"
{@l myvar "valid"} ; sets a variable on the owned scope, and evaluates to null
```
examples of invalid programs.
```
{ ; unexpected opening scope
{}} ; unexpected closing scope
{,anything} ; unexpected identifier ,
```
Some scopes are owning scopes. Owning scopes maintain state. [types](#types), [functions](#functions), [generics](#interfaces-and-generics), and files are an owning scope.

See [Valid Syntax](#valid-syntax) for complete reference on valid program and scope syntax.

---
### Types
Everything that is evaluated has a type. Types are defined using the declaration `@` [identifier](#identifiers).

Type definition.
```
@type computer { 
  mouse = ""
}
```
`mouse` is a [property](#properties).

Types can also have constructor arguments.
```
@type computer(mouseName) {
  mouse = mouseName 
}
```
You can strongly type constructor arguments and properties.
```
@type computer(mouseName: 'string) {
  mouse: 'string = mouseName
}
```
Types can also have no properties, these are `type definitions`.
```
@type special-string: 'string
```
`special-string` is said to be of type `'string`. In the previous example, `@type computer(mouseName)` has no type. It is implied in this scenario that the type returned is `'computer`. This begins getting into [dependent types](#dependent-types) which are a very powerful feature of the language.

For a complete list of types provided in the language see [types list](#types-list)

See [Valid Syntax](#valid-syntax) for complete reference on valid types syntax.

---
#### Variables
Variables are state that is stored on the owning [scope](#program-and-scope). Variables are defined using the declaration `@` [identifiers](#identifiers)

string examples:
``` 
@let mystring1 = ""
@let mystring2 = "this is a string."
@let mystring3: 'string = "strongly typed string"
```

---
number examples:
```
@let mynum1 = 0
@let mynum2 = 0.0
@let mynum3: 'int = -500 // signed integer
```

All of the above are valid ways to declare a number. All numbers without a specific type will be 64 bit floating point. 
It is possible to use other types of numbers, see the complete [types list](#types-list)

---
list examples:
```
@let mylist1 = []
@let mylist3 = [1 2 3]
@let mylist4: 'list = ["my" "list" "four"]
@let mylist5 = ["my" "list" 5]
```

---
null examples:
```
@let myvar1
@let myvar2 = {}
@let myvar3 'null
@let myvar4 'null = {}
``` 
null is both a type, and an evaluation. The first example is of type any. As it could later be assigned. The evaluation of an unassigned variable is null.
See [Default Behavior](#default-behavior) to understand how this execution could impact your project.
  
See [Valid Syntax](#valid-syntax) for complete reference on valid variable syntax.

---
#### Properties
Properties exist only on [types](#types). Since types are an owning [scope](#program-and-scopes), properties can access other properties in the same type.
```
@type computer () {
  monitors =  []
  gpus = []
  peripheral-count = .monitors.length + .gpus.length
}
```

See [Valid Syntax](#valid-syntax) for complete reference on valid property syntax.

---
#### Identifiers
Identifiers are prepended to text. They have special meaning which can quickly inform the `parser` `linter` `compiler` what to expect next. Real performance tests are needed to determine if the performance gain would be worth it. There might be an option to disable/enable in the future.
```
@ # $ % & * ; :  , ... . () ' " ? / | _
```
`@` - used for defining/declaring.

`/` - used for comments, everything up until the next line is immediately ignored. multi-line comments are up to your ide tooling to insert ; on every line to be commented

`#` - used for preprocessor commands.

`{}` - scoping block. A file is an implicit scope block.

`.` - indicates properties on a type. Also used for accessing those properties. and accessing functions declared for a type or interface.

`%` - builtin identifier. These are functions that are embedded in the language, and are specific to the arch + OS.

`'` - type indication identifier.

`$` - used in string templating.

`...` - variadic identifer.

`_` - rest identifier. Used in case matching 

`& * ? / | ` - are reserved for now.

---
#### Functions
Functions have their own [scope](#program-and-scope). They are declared with the declaration `@` [identifier](#identifiers)
```
@fn add (x y) {
  x + y  
}
@fn typed_minus(x: 'i32, y: 'i32): 'i32 {
  x - y
}
```
See [Valid Syntax](#valid-syntax) for complete reference on valid function syntax.

---
#### Builtins

Some functions are built into the language. Here is a list of all.
```
%ffi - used for calling functions local to the Operating System.
%thread - used for creating and working with threads.
%proc - used for creating a process.
%do - used for serially executing functions.
%if - used for conditionallity executing one of several blocks of code.
%while - loops while a condition is true.
%1while - executes the block of code at least once even if condition is false.
%loop - executes consuming an implicit iterator.
%arrow- allows for a function to be projected onto its parameters.
%main - special function which tells the program where to start
%+ - addition.
%- - subtraction.
%% - modulo.
%/ - divide.
%* - multiplication.
%< - less than.
%> - greater than.
%| - or.
%& - and.
%[ - shift left.
%] - shift right.
%~ - not.
%^ - xor.
%! - falsey. works the same as `not` on bits, but handlesnull for types.
%= - equality. same as xor on bits, but handlesnull for types.
```
Every builtin has an evaluation. Builtins are specific implementations for the arch and OS, but are guaranteed to behave the same way on any of the supported.

---
#### Enums
An enumeration is a [type](#types) which evaluates to a different type.
You define an enum with the declaration `@` [identifier](#identifiers)
```
@enum Directions: {
  'NORTH
  'SOUTH
  'EAST
  'WEST
}
```
Defining an enum.
```
@let current-direction1: 'NORTH
@let current-direction2: 'Direction'NORTH
```
When executing code with enums, all possible outcomes must be defined.
Here is an example using the [builtin](#builtins) `%match`.
```
@fn turn-clockwise (myparam: 'Directions)
  %match myparam: { // match implements a yielded type.
    'NORTH: 'EAST 
    'SOUTH: 'WEST
    'EAST: 'SOUTH
    'WEST: 'NORTH
  }
}
```
See [functions](#functions), and [match](#match)

See [Valid Syntax](#valid-syntax) for complete reference on valid enum syntax.

---
#### Interfaces and Generics
Interfaces describe contracts that a [type](#types) must implement in order for them to be used with a generic.
You declare an interface with the declaration `@` [identifer](#identifiers)
```
@interface debug-it (param1) {
  .debug = param1
}

@interface debug-it-strong (param1: 'string) {
  .debug: string = param1
}
```
The first interface can take any type in the `constructor arguments` and ensures that they get assigned to the `.debug` property.
The second interface specifies that this interface is strictly for types that have a `.debug` property, which are of type `'string`. 

See [Valid Syntax](#valid-syntax) for a complete reference on valid interface syntax. 

`constructor arguments` is used loosely here, as there is never going to be an instance of the interface.

```
'debug-it
@type computer {
  .mouse = ""
  .debug = "It's a computer"
}
```
The `computer` type must implement the `.debug` property. Annotating computer in the line above it with `'debug-it` is a contract saying this computer type, implements the debug-it interface.
 
See [Default Behavior](#default-behavior) to understand how interfaces work on interpreted programs.

Generics are special [functions](#functions) which take interfaces and parameters in its constructor arguments.
The properties on the contract can be used by name directly. 
```
@generic print-out (thing: 'debug-it) {
  printf(thing.debug)
}
```
Any type, that implements `'debug-it`, can be passed to the print-out function.

See [Valid Syntax](#valid-syntax) for a complete reference on valid generics syntax.

---
#### Assembly

The assembly syntax follows the new `asm!` syntax for rust as closely as possible. Writing assembly like this is probably one of the most pleasant experiences in writing assembly.
Here is an example.

** WIP **

// the implication of taking a type, and using some sort of reflection around 'reg and inout, and the fact you need to mark the command with #asm indicates this entire scoped block is unrelated to the rest of the language.

// might as well just use (reg), 'reg, or reg?
```
(add2 #asm (x)
  (%addi x x num)
  (%inout ('reg) x) ** wip** <--- what does it mean to take a 'type? is this the instantiation of this type like the rest of the language?
  (%c num 2))
```
First, the function must be marked with the preprocessor command `#asm`. Next we can use the builtins that are made for this specific arch.

`%addi` takes a `dst`, `src`, and `imm`.

`%inout` tells the compiler to use any register it chooses with ('reg), and then x is both an `in` and `out`. The compiler will successfully deduce that it can leave x in the same register, and it will be clobbered. The compiler keeps a list of all registers which are currently in use. if the `%out` or `%inout` is not specified for the variable/register, the compiler will put which ever register contained the variable x, back in the available pool to draw from.

With proper usage of the `%in %out %inout %inlateout %lateout` You are able to squeeze out the best performance possible.

---
#### Types List

---
#### Bultins List

---
#### Valid Syntax

---
# Tutorials


---
# Default Behavior

Wisc is designed to be able to run interpretted or compiled code. This means that if you pass a script to the runner `wiscr` it needs to `just run it`. In order to run the script, it must be a valid program first. See [Program and Scope](#program-and-scope)

We must come to an agreement on what to do when a program is valid, but has undefined behavior. For example, what happens when we try to add `5` and a string `"5"` together? We have two options. We can throw a runtime error, you probably didn't mean to actually do this, unless you were trying to concatenate strings.

Or, we can come to an agreement on the default behavior of the program in this instance. Javascript, for example, concatenates the two values, and returns the string version. `"55"`. 

Let us do one more example. If we had a type `'computer` and wanted to make a list of computers that we have at home. We might write the below code.
```
@type computer() {
  // various properties on a computer
}
@let home_computers = computer()
printf(home_computers.length)
```
Oops, we accidentally just created one computer, it is not a list. We might have intended this declaration instead `@let home_computers = [computer()]`. Should calling `.length` on `home_computers` return `'null`? `0`? `1`?

There are additional tools built on top of wisc that you can use while developing to prevent these kind of misunderstandings, and flat out reject the script in its state. For example, with the linter enabled, we would see this.
```
@type computer() {
  // various properties on a computer
}
@let home_computers = computer()
printf(home_computers.length) // error: property length does not exist on type computer.
```
This will help give feedback to the writer of this code, that they could have undesired behavior in their program. Some might say, well the developer obviously meant that it was supposed to be a list. But unfortunately, we can't make deductions about developer intent, when they have lacked given us the proper context, take this code.

```
@type computer() {
  // various properties on a computer
}
@let favorite_computer = computer()
@let all_computers = [computer()]
printf(favorite_computer.length)
```
They obviously meant for `favorite_computer` to be just one, and they probably meant for the line `printf(favorite_computer.length)` to be `printf(all_computers.length)`. These types of deductions are too difficult to make at execution time of the script. This means that we must make these decisions together, ahead of time. Here is a guide on how the runner will behave.

* The runner will execute in a way that is safest, and least breaking, to runtime execution.
* There are 3 different behind the scenes types. `values`, `lists`, `types`.
* `types` and `lists` have properties. `values` do not.
* All attempts to access a property on a `value` will return `'null`.
* Doing any function, operation, or property access on a variable that is `'null` will also yield `'null`.
* Any list operation performed on a type, will convert it to a list, with itself as the first item in the list.
* Reassignment to a variable, will change its type if different.
* Variables that are used before they are declared, will be declared immediately, and given the value `'null`.


---
# Projects

---
# Embedded

---
# Minifier

---
# Linter

---
# Compiler

---
# Library

---
