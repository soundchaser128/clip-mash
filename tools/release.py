import re
from pathlib import Path
from typing import List
from semver import Version
from bs4 import BeautifulSoup
import subprocess
import sys
import os
import markdown

cargo_toml_path = Path("Cargo.toml")
cargo_toml_version_regex = re.compile(r'^version = "(?P<version>.*)"$', re.MULTILINE)
dry_run = False


class ChangeLogEntry:
    version: str
    entries: List[str]

    def __init__(self, version: str, entries: List[str]):
        self.version = version
        self.entries = entries

    def __str__(self):
        return f"ChangeLogEntry(version={self.version}, entries={self.entries})"

    def markdown(self):
        string = f"## {self.version}\n"
        for entry in self.entries:
            string += f"- {entry}\n"
        string += "\n"
        return string


class ChangeLog:
    entries: List[ChangeLogEntry]

    def __init__(self, entries: List[ChangeLogEntry]):
        self.entries = entries

    @staticmethod
    def parse(path: Path):
        with open(path, "r") as f:
            string = f.read()
        html = markdown.markdown(string)
        soup = BeautifulSoup(html, "html.parser")
        entries = []
        for h2 in soup.find_all("h2"):
            version = h2.text
            entries_html = []
            for sibling in h2.next_siblings:
                if sibling.name == "h2":
                    break
                if sibling.name == "ul":
                    for li in sibling.find_all("li"):
                        entries_html.append(li.text.strip())
            entries.append(ChangeLogEntry(version, entries_html))
        return ChangeLog(entries)

    def __str__(self):
        string = ""
        for entry in self.entries:
            string += f"{entry.version}\n"
            for line in entry.entries:
                string += f"  {line}\n"
        return string

    def markdown(self):
        string = "# Changelog\n"
        for entry in self.entries:
            string += entry.markdown()
        return string

    def new_release(self, version: str):
        unreleased = self.entries[0]
        if unreleased.version != "Unreleased":
            raise Exception("No unreleased section found")
        new_version = ChangeLogEntry(version, unreleased.entries)
        self.entries.insert(1, new_version)
        unreleased.entries = []


def cmd(command: List[str]):
    if dry_run:
        print(" ".join(command))
    else:
        result = subprocess.run(command)
        if result.returncode != 0:
            raise RuntimeError(f"Command failed: {command}: {result.returncode}")


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


def main():
    args = sys.argv[1:]
    if len(args) != 1:
        print("Usage: release.py <pre|patch|minor|major>")
        sys.exit(1)

    type = args[0]
    os.chdir("backend")
    try:
        new_version = update_cargo_toml(type)
        if type != "pre":
            change_log = ChangeLog.parse(Path("../CHANGELOG.md"))
            change_log.new_release(str(new_version))
            with open(Path("../CHANGELOG.md"), "w") as f:
                f.write(change_log.markdown())

        # Run cargo check to update Cargo.lock
        cmd(["cargo", "check"])

        # Push a commit to the repo with the new version
        cmd(["git", "add", ".."])
        cmd(["git", "commit", "-m", f"chore: Prepare release {new_version}"])
        cmd(["git", "push", "--no-verify"])

        # Create a tag
        cmd(["git", "tag", f"v{new_version}"])
        # Push the tag
        cmd(["git", "push", "--tags", "--no-verify"])
    finally:
        os.chdir("..")


if __name__ == "__main__":
    main()
