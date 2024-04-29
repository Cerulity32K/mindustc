# MindustC
A C-like language that transpiles into Mindustry assembly.

Keep in mind this is my first language. Any feedback is welcome.

# Syntax
Basic C-like syntax is supported. For now, only expressions are allowed.
Expressions wrapped in dollar signs ($) are interpreted as inline logic and are passed through to output as-is.

Currently, there are no if statements, functions, comments, structs, or any sort of preprocessor,
but there are plans to implement them in the future.

# Examples
Input:
```
thing = -50;
cool = thing * 2 - 20;
$asdfghjkl$
```
Output:
```
set thing -50
op mul cool thing 2
op sub cool cool 20
asdfghjkl
```
# Status
As of now, this project has been ***discontinued***. However, expect a new transpiler soon. I'm rewriting and redesigning this project to create Scriptdustry.