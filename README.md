# 😌 Sane Shell

> A modern language compiled to Bash

```js
read!(`-p "N = " N`)

let a = 0
let b = 1
let i = 0

while (i < N) {
    let temp = b
    b = a + b
    a = temp

    i = i + 1
}

print("fibb($N) = $a") 
```

## 💬 Introduction

...

## 📖 Syntax

#### Comments

You can specify single line comments using `#` like in shell.

```sh
# This is a comment
```

#### Variables

To declare a variable use `let` keyword followed by an identifier.\
The name needs to be alphanumerical with underscores but needs to start with a letter.

```js
let a = 0
let b = a + 3
```

#### Conditionals

...

```shell
if (a > 5) {
    # This is a comment
}
```

### Loops

#### While

```js
let i = 0
if (i < 5) {
    i = i+1
}
```

#### 🚧 For 🚧

```sh

for 0..10 {
    # Do something
}

# or

for i in 0..10 {
    # Do something with i
}

# or

for x in arr {
    # Do something with x
}
```

### Functions and commands

#### Builtin functions

```sh
# Print output to screen
print("Hello from Sash!")
# Create an archive 'archive.tar.gz' from files 'file.txt' and 'another.jpg'
compress("file.txt", "another.jpg", "archive.tar.gz")
# Decompress an archive file 'archive.tar.gz'
decompress("archive.tar.gz")
```

#### Commands

You can invoke any process by following its name with `!`.

```js
echo!("Hello using standard echo!")
tar!("-caf", "file.txt", "another.jpg", "archive.tar.gz")
tar!("-xf", "archive.tar.gz")
```

#### 🚧 Custom functions 🚧

You can define functions just like you can in Bash. The difference is that you have to specify the parameters.

```js
function pretty_print(msg) {
    print(green("log:"), $msg)
}

pretty_print("Hello from Sash!")
```

## ⚖️ License

[MIT](LICENSE)
