import argparse
import logging
import os
from enum import Enum
from pathlib import Path

import semver
import toml
from semver.version import Version
from sh import git


class BumpType(Enum):
    MAJOR = "major"
    MINOR = "minor"
    PATCH = "patch"

    @classmethod
    def choices(cls) -> list[str]:
        return [member.value for member in cls]


def _file(wd: Path, fname: str) -> Path:
    if not (f_path := wd / fname).is_file():
        raise FileNotFoundError(f"No file '{fname}' found in '{wd}'")
    return f_path


def get_cargo_lock_package_version(rust_directory: Path, package_name: str) -> Version:
    cargo_lock = _file(rust_directory, "Cargo.lock")
    d = toml.load(cargo_lock)
    filtered = [y["version"] for y in d["package"] if y["name"] == package_name]
    match filtered:
        case [one_hit]:
            cargo_lock_version = str(one_hit)
        case _:
            raise ValueError("")
    return semver.Version.parse(cargo_lock_version)


def get_cargo_toml_package_version(cargo_toml: Path) -> tuple[Version, str]:
    cargo_toml = _file(rust_directory, "Cargo.toml")
    d = toml.load(cargo_toml)
    try:
        cargo_toml_version = str(d["package"]["version"])
        cargo_toml_name = str(d["package"]["name"])
    except KeyError as err:
        raise ValueError(f"Invalid Cargo.toml file: '{cargo_toml}'") from err

    return semver.Version.parse(cargo_toml_version), cargo_toml_name


def create_and_push_tag(ver: Version) -> None:
    tag = f"v{ver}"

    log.info(f"Creating tag {tag}...")
    git("tag", "-am", tag, tag)
    git("push", "origin", tag)


def main(
    version: str, bump_type: BumpType, rust_directory: Path, dry_run: bool
) -> None:
    cargo_toml_version, package_name = get_cargo_toml_package_version(rust_directory)
    cargo_lock_version = get_cargo_lock_package_version(rust_directory, package_name)

    ver = semver.Version.parse(version)
    match bump_type:
        case BumpType.MAJOR:
            ver = ver.bump_major()
        case BumpType.MINOR:
            ver = ver.bump_minor()
        case BumpType.PATCH:
            ver = ver.bump_patch()

    if ver != cargo_lock_version:
        raise ValueError(
            f"New version ({ver}) != Cargo Lock Version ({cargo_lock_version}). Please update Cargo.lock."
        )
    if ver != cargo_toml_version:
        raise ValueError(
            f"New version ({ver}) != Cargo TOML Version({cargo_toml_version}). Please update Cargo.toml."
        )
    if cargo_lock_version != cargo_toml_version:
        raise ValueError(
            f"Version mismatch: [Cargo Lock Version] {cargo_lock_version} != [Cargo TOML Version] {cargo_toml_version}"
        )

    if dry_run is True:
        log.info("Running in dry mode, skipping release.")
        return

    match input(
        "Confirm Cargo.toml and Cargo.lock updates have been committed (y/n):"
    ).lower():
        case "y":
            pass
        case _:
            exit(1)

    match input(f"Confirm release {ver} (y/n):").lower():
        case "y":
            create_and_push_tag(ver)
        case _:
            exit(1)


def check_directory(rust_directory: str | None) -> Path:
    match rust_directory:
        case None:
            directory = Path(os.getcwd())
        case _ as valid_directory:
            if not (directory := Path(valid_directory)).is_dir():
                raise NotADirectoryError(
                    f"'{valid_directory}' is not a valid directory"
                )

    return directory


def ensure_main():
    if git("branch", "--show-current").strip() != "main":
        raise ValueError("Release tags can only be created on the main branch")


if __name__ == "__main__":
    logging.basicConfig(level=logging.INFO)
    log = logging.getLogger(__name__)

    parser = argparse.ArgumentParser(description="Release a new version of the project")
    parser.add_argument(
        "--current_version", type=str, help="Current semantic version.", required=True
    )
    parser.add_argument(
        "--bump_type", type=str, choices=BumpType.choices(), required=True
    )
    parser.add_argument(
        "-d", "--directory", help="Path to Rust project directory.", required=False
    )
    parser.add_argument("--dry_run", help="", action="store_true")

    args = parser.parse_args()

    rust_directory = check_directory(args.directory)
    bump_type = BumpType(args.bump_type)
    dry_run = bool(
        args.dry_run,
    )

    ensure_main()

    main(args.current_version, bump_type, rust_directory, dry_run)
