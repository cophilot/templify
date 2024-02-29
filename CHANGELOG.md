# CHANGELOG _templify_

---

## [v0.7.0](https://github.com/cophilot/templify/tree/0.7.0) (2024-2-29)

-   macOS support added

---

## [v0.6.0](https://github.com/cophilot/templify/tree/0.6.0) (2024-2-20)

-   Refactoring
-   Added placeholder `$$month-name$$`
-   Added case conversion support for the `name` placeholder

---

## [v0.5.0](https://github.com/cophilot/templify/tree/0.5.0) (2024-2-17)

-   Refactoring
-   Added `reload` command
-   Added `-name` flag for the `list` command
-   Added `-path` flag for the `list` command
-   Added `-template` flag for the `load` command
-   Support for `.tpykeep` file to prevent a directory from being deleted

---

## [v0.4.1](https://github.com/cophilot/templify/tree/0.4.1) (2024-2-14)

-   Refactoring
-   `.source` attribute will be set in the `.templify` file when a template is loaded

---

## [v0.4.0](https://github.com/cophilot/templify/tree/0.4.0) (2024-2-12)

-   `.templify` file is now optional
-   Added `-blank` flag for the `init` command
-   Added placeholder `$$year$$`
-   Added placeholder `$$month$$`
-   Added placeholder `$$day$$`
-   Added placeholder `$$git-name$$`

---

## [v0.3.1](https://github.com/cophilot/templify/tree/0.3.1) (2024-2-9)

-   Added `-dry-run` flag for the `generate` command

---

## [v0.3.0](https://github.com/cophilot/templify/tree/0.3.0) (2024-2-6)

-   Bug fixes
-   Added `command` argument for the `help` command to display help for a specific command
-   Command `generate` uses pattern matching to determine the type of template to generate
-   Added `-strict` flag for `generate` command to disable pattern matching

---

## [v0.2.2](https://github.com/cophilot/templify/tree/0.2.2) (2024-2-5)

-   Added `-force` flag for `load` command
-   Added `-version` flag for `update` command

---

## [v0.2.1](https://github.com/cophilot/templify/tree/0.2.1) (2024-2-4)

-   Refactoring
-   Added `-offline` flag for `init` command
-   Added `-description` flag for `new` command
-   Added `-path` flag for `new` command

---

## [v0.2.0](https://github.com/cophilot/templify/tree/0.2.0) (2024-1-30)

-   New logo
-   Added `load` command
-   Check for internet connection before fetching resources

---

## [v0.1.0](https://github.com/cophilot/templify/tree/0.1.0) (2024-1-29)

-   Added self updating feature
-   Minor design changes
-   Bug fixes

---

## [v0.0.1](https://github.com/cophilot/templify/tree/0.0.1) (2024-1-28)

-   _Initial release_

---
