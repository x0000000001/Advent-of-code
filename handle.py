"""
Handler to create and move aoc files.
"""

import sys
import os
import shutil
from datetime import datetime

current_day_folder = "current_day/"
if not os.path.exists(current_day_folder):
    os.mkdir(current_day_folder)

metadata_name = ".aoc_data"
templates_folder = "templates/"
accepted_languages = [s.split("/")[-1][3:] for s in os.listdir(templates_folder)]
help_string = f"""
Utility to create, edit, and save aoc days in this repository.

Usage:
    - create <year> <day> <language> 
    - load <year> <day> <language>
    - save

      e.g. 

      "py handle.py load 2018 07 rust"

Accepted languages:
    {accepted_languages}
"""


def get_day_path(language, year, day):
    if language == "rust":
        raise Exception("Rust should not be handled here")

    return f"{language}/{year}/day{day}_{language}/"


def rust_save_day(current_folder, year, day):
    lib_path = f"{current_folder}/src/lib.rs"

    if os.path.exists(lib_path):
        print(f"{lib_path} already exists, do you want to overwrite it ? (y/n)")
        answer = input()
        if answer != "y":
            print("Aborting.")
            return

    with open(lib_path, "r") as f:
        lines = f.readlines()
    lines = [
        line
        for line in lines
        if not (
            line.startswith("mod helpers;")
            or line.startswith("pub use helpers::Solution;")
        )
    ]
    lines.insert(0, "use crate::Solution")
    day_path = f"rust/src/year_{year}/day{day}.rs"
    mod_path = f"rust/src/year_{year}/mod.rs"

    with open(day_path, "w") as f:
        f.writelines(lines)

    with open(mod_path, "r") as f:
        needs_day_mod = f"day{day}" not in f.read()

    if needs_day_mod:
        with open(mod_path, "a") as f:
            f.write("\n\nmod day{day};\n")

    shutil.rmtree(current_folder)
    os.mkdir(current_folder)


def create_day(language, year, day):
    template_dir = f"{templates_folder}/day{language}/"

    if not os.path.exists(current_day_folder):
        raise Exception(f"No template for language {language}")

    shutil.copytree(template_dir, current_day_folder, dirs_exist_ok=True)
    metadata_file = current_day_folder + metadata_name
    with open(metadata_file, "w") as f:
        f.write(f"{year}\n{day}\n{language}\n")


def save_day(current_folder):
    with open(current_folder + metadata_name, "r") as f:
        year, day, language = f.readlines().strip()

    if language == "rust":
        rust_save_day(current_folder, year, day)
        return

    day_path = get_day_path(language, year, day)
    if os.path.exists(day_path):
        print(f"Folder {day_path} already exists, do you want to overwrite it ? (y/n)")
        answer = input()
        if answer != "y":
            print("Aborting.")
            return

    os.remove(current_folder + metadata_name)
    shutil.move(current_folder, day_path)
    os.mkdir(current_folder)


def retrieve_args():
    if len(sys.argv) != 5:
        print(f"Only {len(sys.argv) -1} arguments detected.")
        print(help_string)
        return None, None, None

    try:
        language = sys.argv[4]
        year = int(sys.argv[2])
        day = sys.argv[3]
        assert language in accepted_languages, print(f"{language} is not accepted")
        day_int = int(day)
        assert day_int > 0 and day_int < 26, print("Day should be between 1 and 25")
        assert year > 2014 and year <= datetime.now().year, print(
            "Year should be valid"
        )
    except:  # noqa: E722
        print(help_string)
        return (
            None,
            None,
            None,
        )

    return language, year, day


def does_day_exists(language, year, day):
    if language == "rust":
        return os.path.exists(f"rust/src/year_{year}/day{day}.rs")
    else:
        return os.path.exists(get_day_path(language, year, day))


def load_day(language, year, day):
    if language == "rust":
        rust_load_day(year, day)

    day_path = get_day_path(language, year, day)
    if not os.path.exists(day_path):
        raise Exception(f"Folder {day_path} doesn't exists, aborting.")

    shutil.copytree(day_path, current_day_folder)


def rust_load_day(year, day):
    lib_path = f"rust/src/year_{year}/day{day}.rs"
    input_folder_path = f"rust/data/year_{year}/day{day}/"

    with open(lib_path, "r") as f:
        lines = f.readlines()

    lines = [line for line in lines if not (line.startswith("use crate::Solution"))]
    lines.insert(0, "mod helpers;")
    lines.insert(1, "pub use helpers::Solution;")

    create_day("rust", year, day)
    os.remove(current_day_folder + "src/lib.rs")
    os.remove(current_day_folder + "test_input0.txt")
    with open(current_day_folder + "src/lib.rs", "w") as f:
        f.writelines(lines)
    shutil.copytree(input_folder_path, current_day_folder)


def assert_edit_folder_empty():
    # a day is already being edited
    files_in_folder = os.listdir(current_day_folder)

    if not files_in_folder:
        return True
    metadata_file = current_day_folder + metadata_name

    if os.path.exists(metadata_file):
        with open(metadata_file, "r") as f:
            year, day, language = f.readlines()

        print("A day is already present in the folder :")
        print(f"Year: {year}")
        print(f"Day: {day}")
        print(f"Language: {language}")
        print("Do you want to move it first (y) or abort (n) ?")
        answer = input()
        if answer == "y":
            save_day(current_day_folder)
            return True
        else:
            return False
    else:
        print(
            f"{current_day_folder} is not empty and doesn't contain {metadata_name} file."
        )
        print("Please empty it first.")
        return False


def command_create():
    language, year, day = retrieve_args()
    if not language and assert_edit_folder_empty():
        return

    if does_day_exists(language, year, day):
        print("This day already exists, do you want to load it ? (y/n)")
        answer = input()
        if answer == "y":
            load_day(language, year, day)
            return

    create_day(language, year, day)


def command_load():
    language, year, day = retrieve_args()
    if not language and assert_edit_folder_empty():
        return

    if not does_day_exists(language, year, day):
        print("This day doesn't exists, aborting.")
        return

    load_day(language, year, day)


def command_save():
    files_in_folder = os.listdir(current_day_folder)

    if not files_in_folder:
        print("No day is being currently edited, aborting.")
        return

    metadata_file = current_day_folder + metadata_name

    if not os.path.exists(metadata_file):
        print(f"Can't find {metadata_file} : please save day by hand.")
        return

    save_day(current_day_folder)


if __name__ == "__main__":
    match sys.argv[1]:
        case "create":
            command_create()
        case "load":
            command_load()
        case "save":
            command_save()
        case _:
            print(help)

# TODO command create_today with automatic input fetch
