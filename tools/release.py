import re
from pathlib import Path
from semver import Version
import subprocess
import os
import sys
import markdown

cargo_toml_path = Path("Cargo.toml")
cargo_toml_version_regex = re.compile(r'^version = "(?P<version>.*)"$', re.MULTILINE)
dry_run = False


def cmd(command):
    if dry_run:
        print(" ".join(command))
    else:
        subprocess.run(command)


def update_cargo_toml(type: str):
    with open(cargo_toml_path, "r") as f:
        cargo_toml = f.read()

    match = cargo_toml_version_regex.search(cargo_toml)
    if match is None:
        raise Exception("Could not find version in Cargo.toml")

    version = match.group("version")
    version = Version.parse(version)
    print(f"Current version: {version}")

    if type == "pre":
        if version.prerelease is None:
            version = version.bump_minor()
        version = version.bump_prerelease("pre")
    elif type == "patch":
        version = version.bump_patch()
    elif type == "minor":
        version = version.bump_minor()
    elif type == "major":
        version = version.bump_major()
    else:
        raise Exception(f"Unknown version type: {type}")

    print(f"New version: {version}")

    cargo_toml = cargo_toml_version_regex.sub(f'version = "{version}"', cargo_toml)

    with open(cargo_toml_path, "w") as f:
        f.write(cargo_toml)
    return version


def update_changelog():
    # find every entry under the "Unreleased" heading
    with open("CHANGELOG.md", "r") as f:
        changelog = f.read()
    document = markdown.markdown(changelog)
    print(document)
    unreleased = document.find("Unreleased")
    if unreleased == -1:
        raise Exception("Could not find Unreleased section in CHANGELOG.md")
    next_heading = document.find("<h2>", unreleased + 1)
    if next_heading == -1:
        next_heading = len(document)
    entries = document[unreleased:next_heading]
    # print(entries) 


def main():
    args = sys.argv[1:]
    if len(args) != 1:
        print("Usage: release.py <pre|patch|minor|major>")
        sys.exit(1)

    type = args[0]

    update_changelog()

    # os.chdir("backend")
    # try:
    #     new_version = update_cargo_toml(type)
    #     if type != "pre":
    #         update_changelog()

    #     # Run cargo check to update Cargo.lock
    #     cmd(["cargo", "check"])

    #     # Push a commit to the repo with the new version
    #     cmd(["git", "add", ".."])
    #     cmd(["git", "commit", "-m", f"chore: Prepare release {new_version}"])
    #     cmd(["git", "push"])

    #     # Create a tag
    #     cmd(["git", "tag", f"v{new_version}"])
    #     # Push the tag
    #     cmd(["git", "push", "--tags"])
    # finally:
    #     os.chdir("..")


if __name__ == "__main__":
    main()
