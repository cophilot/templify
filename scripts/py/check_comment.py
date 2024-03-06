import os

DIRECTORY = ["./src"]

FILE_TYPES = ["rs"]

def main():
    print("Checking for missing comments...")
    print()
    error_detected = False
    for dir in DIRECTORY:
        if not check_dir(dir):
            error_detected = True
    if error_detected:
        exit(1)
    print("No missing comments found!")

def check_dir(directory):
    return_value = True
    
    files = os.listdir(directory)
    for file in files:
        if os.path.isdir(os.path.join(directory, file)):
            if not check_dir(os.path.join(directory, file)):
                return_value = False
        else:
            if not check_file(os.path.abspath(os.path.join(directory, file))):
                return_value = False
    return return_value

def check_file(file):
    if not (file.split(".")[-1] in FILE_TYPES):
        return True
    return_value = True
    with open(file, "r") as f:
        lines = f.readlines()
        for i in range(0, len(lines)):
            line = lines[i]
            if line.strip().startswith("//") or line.strip().startswith("/*") or line.strip().startswith("*") or line.strip().startswith("*/"):
                continue
            if not "fn" in line.split(" ") and not "struct" in line.split(" "):
                continue
            if i == 0:
                return_value = False
                print_error(file, i + 1)
                continue
            line_before = lines[i - 1]
            if (not "///" in line_before):
                return_value = False
                print_error(file, i + 1)
    return return_value

def print_error(file, line):
    path = os.path.abspath(file)
    print(f"Missing Comment: {path}, line {str(line)}")

if __name__ == "__main__":
    main()