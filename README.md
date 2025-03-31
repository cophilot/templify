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
   <a href="https://github.com/cophilot/templify/actions/workflows/check_rust_quality.yml">
       <img src="https://img.shields.io/github/actions/workflow/status/cophilot/templify/check_rust_quality.yml?style=flat&logo=Rust&label=CI" alt="CI-Pipeline" />
   </a>
   <a href="https://github.com/cophilot/templify/blob/main/LICENSE">
       <img src="https://img.shields.io/github/license/cophilot/templify" alt="license" />
   </a>
   <a href="https://github.com/cophilot/templify/commits/main">
       <img src="https://img.shields.io/github/last-commit/cophilot/templify" alt="last commit" />
   </a>
   <a href="https://templify.philipp-bonin.com/">
       <img src="https://img.shields.io/badge/docs-visit-yellow" alt="docs" />
   </a>
   <a href="https://github.com/cophilot/templify/stargazers">
       <img src="https://img.shields.io/github/stars/cophilot/templify" alt="stars" />
   </a>
</div>

---

For a more detailed documentation visit the [templify-docs](https://templify.philipp-bonin.com/).

---

-   [Concept](#concept)
-   [Installation](#installation)
    -   [Linux / macOS](#linux--macos)
    -   [Windows](#windows)
-   [Docker](#docker)
-   [Development](#development)
    -   [Installation](#installation-1)
    -   [CI Pipeline](#ci-pipeline)
-   [Templates](#templates)
-   [.templify.yml](#templifyyml)
-   [Usage](#usage)
    -   [help](#help)
    -   [version](#version)
    -   [init](#init)
    -   [new](#new)
    -   [load](#load)
    -   [list](#list)
    -   [generate](#generate)
-   [Placeholders](#placeholders)
    -   [Case conversion](#case-conversion)
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

### Linux / macOS

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

## Docker

You can run an docker container with an installed templify version to try out the features. Just run the following commands:

```
git clone https://github.com/cophilot/templify.git
cd templify
docker build . --tag templify-image
docker run -it templify-image bash
```

---

## Development

### Installation

You can either run the CLI tool directly on your machine or use the dev-container.

#### Local

1. Clone the repository
2. Run `./scripts/setup`
3. Now you can run `cargo run <arguments> --dev` to run the CLI tool

#### Dev-Container

TODO

### CI Pipeline

The CI pipeline ensures the quality of the code. It runs the following checks:

-   **Format Check**: Checks if the code is formatted correctly (Script: [format-check](scripts/format-check))
-   **Comment Check**: Checks if the code contains enough comments (Script: [comment-check](scripts/comment-check))
-   **Linter**: Checks if the code is linted correctly (Script: [lint](scripts/lint))
-   **Test**: Runs the test suite (Script: [test](scripts/test))

> You can find the pipeline configuration in the [.github/workflows/check_rust_quality.yml](.github/workflows/check_rust_quality.yml) file.

---

## Templates

A template is a folder in the `.templates` directory of your project. It contains all files and subfolders that should be generated when the template is used. You can use [placeholders](#placeholders) to replace parts of the template with the given values.
Each template must contain a `.templify.yml` file that specifies some metadata about the template.

---

## .templify.yml

The `.templify.yml` file contains metadata about the template. It is a YAML file with the following keys:

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

-   `$$name$$`: The name of the new file (This placeholder supports case conversion).
-   `$$year$$`: The current year.
-   `$$month$$`: The current month as a number.
-   `$$month-name$$`: The current month as a name.
-   `$$day$$`: The current day.
-   `$$git-name$$`: The name of the git user.

### Case conversion

Case conversion is used to convert placeholders to different case styles. You can use them by adding a `.` and the case style to a placeholder that supports case conversion.

_Example: `$$name.kebab$$`_

You can use the following case conversion:

-   `$$<placeholder>.lower$$`: Lower case (e.g. `mycomponent`)
-   `$$<placeholder>.upper$$`: Upper case (e.g. `MYCOMPONENT`)
-   `$$<placeholder>.camel$$`: Camel case (e.g. `myComponent`)
-   `$$<placeholder>.snake$$`: Snake case (e.g. `my_component`)
-   `$$<placeholder>.kebab$$`: Kebab case (e.g. `my-component`)
-   `$$<placeholder>.pascal$$`: Pascal case (e.g. `MyComponent`)
-   `$$<placeholder>.macro$$`: Macro case (e.g. `MY_COMPONENT`)
-   `$$<placeholder>.train$$`: Train case (e.g. `My-Component`)

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

## [v2.0.1](https://github.com/cophilot/templify/milestone/2)

-   Bug fixes [#20](https://github.com/cophilot/templify/issues/20)

---

Implemented with **Rust 1.82.0** 🦀

---

by [Philipp B.](https://github.com/cophilot)
