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
-   [Template](#template)
-   [.templify](#templify)
-   [Usage](#usage)
-   [Placeholders](#placeholders)
-   [Bugs](#bugs)

---

## Concept

Working on a project often requires the creation of files with a similar structure. For example, a React component often consists of a `.tsx` file, a `.scss` file and a `.test.tsx` file. templify allows you to create templates for such files and generate them from the command line.
It also allows you to specify the location of the generated files to keep your project structure clean.
You can see a real world example [here](https://github.com/cophilot/templify-docs/tree/main/.templates).

---

## Installation

### Linux

Run the following command in your terminal:

```bash
curl -s https://raw.githubusercontent.com/cophilot/templify/master/install | sudo bash
```

or download the binary from the [latest release](https://github.com/cophilot/templify/releases/latest) and place it in your `$PATH`.

After installation run `tpy version` to verify that the installation was successful.

### Windows

Download the binary from the [latest release](https://github.com/cophilot/templify/releases/latest) and add it to your [Path](https://medium.com/@kevinmarkvi/how-to-add-executables-to-your-path-in-windows-5ffa4ce61a53).

After installation run `tpy version` to verify that the installation was successful.

---

## Template

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

## Bugs

-   _no known bugs_

---

## [Release Notes](https://github.com/cophilot/templify/blob/master/CHANGELOG.md)

### [v0.1.0](https://github.com/cophilot/templify/tree/0.1.0)

-   Added self updating feature
-   Minor design changes
-   Bug fixes

---

by [Philipp B.](https://github.com/cophilot)
