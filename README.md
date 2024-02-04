<div align="center">
  <br />
  <img src="assets/logo.png" alt="templifyLogo" width="30%"/>
  <h1>templify</h1>
  <p>
     A CLI tool to keep track of templates and generate files from them.
  </p>
</div>

<!-- Badges -->
<div align="center">
   <a href="https://github.com/cophilot/templify/releases">
       <img src="https://img.shields.io/github/v/release/cophilot/templify?display_name=tag" alt="current realease" />
   </a>
   <a href="https://github.com/cophilot/templify/blob/main/LICENSE">
       <img src="https://img.shields.io/github/license/cophilot/templify" alt="license" />
   </a>
   <a href="https://github.com/cophilot/templify/commits/main">
       <img src="https://img.shields.io/github/last-commit/cophilot/templify" alt="last commit" />
   </a>
   <a href="https://github.com/cophilot/templify/stargazers">
       <img src="https://img.shields.io/github/stars/cophilot/templify" alt="stars" />
   </a>
</div>

---

-   [Concept](#concept)
-   [Installation](#installation)
    -   [Linux](#linux)
    -   [Windows](#windows)
-   [Templates](#templates)
-   [.templify](#templify)
-   [Usage](#usage)
    -   [help](#help)
    -   [version](#version)
    -   [init](#init)
    -   [new](#new)
    -   [load](#load)
    -   [list](#list)
    -   [generate](#generate)
-   [Placeholders](#placeholders)
-   [templify-vault](#templify-vault)
-   [Bugs](#bugs)
-   [Release Notes](#release-notes)

---

## Concept

Working on a project often requires the creation of files with a similar structure. For example, a React component often consists of a `.tsx` file, a `.scss` file and a `.test.tsx` file. templify allows you to create templates for such files and generate them from the command line.
It also allows you to specify the location of the generated files to keep your project structure clean.
You can see a real world example [here](https://github.com/cophilot/templify-docs/tree/main/.templates).

---

## Installation

### Linux

Run the following command in your terminal to isntall the latest version of templify:

```bash
curl -s https://raw.githubusercontent.com/cophilot/templify/master/install | bash -s -- -y
```

Optionally you can specify a version with the `-v` flag:

```bash
curl -s https://raw.githubusercontent.com/cophilot/templify/master/install | bash -s -- -y -v <version>
```

You can also define the installation path with the `-p` flag:

```bash
curl -s https://raw.githubusercontent.com/cophilot/templify/master/install | bash -s -- -y -p /usr/local/bin
```

or download the binary from the [latest release](https://github.com/cophilot/templify/releases/latest) and place it in your `$PATH`.

You may need to restart your terminal after installation for the changes to take effect.

> After installation run `tpy version` to verify that the installation was successful.

### Windows

1. Download the `tpy.exe` binary from the [latest release](https://github.com/cophilot/templify/releases/latest)
2. Create a root folder for templify, e.g. `C:\Program Files\templify`
3. Inside the root folder create a folder called `bin`
4. Place the `tpy.exe` binary inside the `bin` folder
5. Add the `bin` folder to your system's [PATH variable](https://medium.com/@kevinmarkvi/how-to-add-executables-to-your-path-in-windows-5ffa4ce61a53)

> After installation run `tpy version` to verify that the installation was successful.

---

## Templates

A template is a folder in the `.templates` directory of your project. It contains all files and subfolders that should be generated when the template is used. You can use [placeholders](#placeholders) to replace parts of the template with the given values.
Each template must contain a `.templify` file that specifies some metadata about the template.

---

## .templify

The `.templify` file contains metadata about the template. It is a key-value file with the following keys:

-   `description`: A short description of the template
-   `path`: The path where the generated files should be placed based on the root of the project. This can also be a path with [placeholders](#placeholders) in it.

---

## Usage

```bash
tpy [command]
```

Commands:

### help

```bash
tpy help
```

Displays help for templify.

### version

```bash
tpy version
```

Displays the current version of templify.

### init

```bash
tpy init
```

Initializes templify in the current directory.

### new

```bash
tpy new <template-name>
```

Creates a new template with the given name.

### load

```bash
tpy load <url>
```

Load predefined templates from a GitHub repository into your project. The url should point to a folder inside a GitHub repository. Each folder inside there will be imported as a template in your _.templates_. See the [templify-vault](#templify-vault) for more informations.

### list

```bash
tpy list
```

Lists all available templates in the current project.

### generate

```bash
tpy generate <template-name> <given-name>
```

Generates a file from the given template.

---

## Placeholders

Placeholders are used to replace parts of the template with the given values. They are defined by surrounding a word with two dollar signs (`$$`):

-   `$$name$$` will be replaced with the given name

---

## templify-vault

The [templify-vault](https://github.com/cophilot/templify-vault) repository contains some default templates that you can import with the `load` command.

**_Example:_**

You wanna start a new [React](https://react.dev/) project with [typescript](https://www.typescriptlang.org)? Just run this command to import templates for default components for you React app:

```bash
tpy load https://github.com/cophilot/templify-vault/tree/main/React-ts
```

---

## Bugs

-   _no known bugs_

---

## [Release Notes](https://github.com/cophilot/templify/blob/master/CHANGELOG.md)

### [v0.2.1](https://github.com/cophilot/templify/tree/0.2.1)

-   Refactoring
-   Added `-offline` flag for `init` command
-   Added `-description` flag for `new` command
-   Added `-path` flag for `new` command

---

by [Philipp B.](https://github.com/cophilot)
