import re
from pathlib import Path
from semver import Version
import subprocess
import os

cargo_toml_path = Path("Cargo.toml")
cargo_toml_version_regex = re.compile(r'^version = "(?P<version>.*)"$', re.MULTILINE)
dry_run = False


def cmd(command):
    if dry_run:
        print(" ".join(command))
    else:
        subprocess.run(command)


def update_cargo_toml():
    with open(cargo_toml_path, "r") as f:
        cargo_toml = f.read()

    match = cargo_toml_version_regex.search(cargo_toml)
    if match is None:
        raise Exception("Could not find version in Cargo.toml")

    version = match.group("version")
    version = Version.parse(version)
    print(f"Current version: {version}")
    if version.prerelease is None:
        version = version.bump_minor()

    version = version.bump_prerelease("pre")
    print(f"New version: {version}")

    cargo_toml = cargo_toml_version_regex.sub(f'version = "{version}"', cargo_toml)

    with open(cargo_toml_path, "w") as f:
        f.write(cargo_toml)
    return version


def main():
    os.chdir("backend")
    try:
        new_version = update_cargo_toml()
        # Run cargo check to update Cargo.lock
        cmd(["cargo", "check"])
        # Run git add .
        cmd(["git", "add", "."])
        # Push a commit to the repo with the new version
        cmd(["git", "commit", "-m", f"chore: Bump version to {new_version}"])
        cmd(["git", "push"])
        # Create a tag
        cmd(["git", "tag", f"v{new_version}"])
        # Push the tag
        cmd(["git", "push", "--tags"])
    finally:
        os.chdir("..")


if __name__ == "__main__":
    main()
