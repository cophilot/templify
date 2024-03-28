""""
This script checks for missing comments in the specified directories.
"""

import os

DIRECTORY = ["./src"]

FILE_TYPES = ["rs"]

KEYWORDS = ["fn", "struct"]

IGNORE_LINES = ["//", "/*", "*/", "*"]

def main():
    """
    This function checks for missing comments in the specified directories.
    If any missing comments are found, it exits with a status code of 1.
    """

    error_count = 0

    for current_dir in DIRECTORY:
        print()
        print(f"Checking for missing comments in {current_dir}...")
        error_count += check_dir(current_dir)

    if error_count > 0 :
        print()
        print(f"🚨 {error_count} missing comments found!")
        exit(1)

    print("✅ No missing comments found!")

def check_dir(directory: str) -> int:
    """
    Recursively checks all files and directories within the given directory.

    Args:
        directory (str): The path to the directory to be checked.

    Returns:
        bool: True if all files and directories within the given directory pass the check, False otherwise.
    """

    error_sum = 0

    try:
        files = os.listdir(directory)
    except FileNotFoundError:
        print(f"ERROR: Directory {directory} not found!")
        exit(1)

    for file in files:
        if os.path.isdir(os.path.join(directory, file)):
            error_sum += check_dir(os.path.join(directory, file))
        else:
            error_sum += check_file(os.path.abspath(os.path.join(directory, file)))

    return error_sum

def check_file(file) -> int:
    """
    Checks if a file contains missing comments.

    Args:
        file (str): The path to the file to be checked.

    Returns:
        int: The number of missing comments found in the file.
    """

    if not file.split(".")[-1] in FILE_TYPES:
        return 0

    error_count = 0
    
    with open(file, "r", encoding="utf-8") as f:

        lines = f.readlines()

        for (i,line) in enumerate(lines):

            fline = line.strip()

            ignore_line = False
            for ignore in IGNORE_LINES:
                if fline.startswith(ignore):
                    ignore_line = True
                    break

            if ignore_line:
                continue

            words = fline.split(" ")

            found_keyword = False

            for keyword in KEYWORDS:
                if keyword in words:
                    found_keyword = True
                    break

            if not found_keyword:
                continue

            line_before = ""
            if i != 0:
                line_before = lines[i - 1]

            if  "///" not in line_before:
                error_count += 1
                print_error(file, i + 1)
    return error_count

def print_error(file, line):
    """
    Prints an error message indicating a missing comment.

    Args:
        file (str): The file path where the missing comment occurred.
        line (int): The line number where the missing comment occurred.
    """
    path = os.path.abspath(file)
    print(f"Missing Comment: {path}, line {str(line)}")

if __name__ == "__main__":
    main()
