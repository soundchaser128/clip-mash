set shell := ["nu", "-c"]

project := justfile_directory()
frontend := project + "/frontend"
backend := project + "/crates/clip-mash-server"

default:
  @just --list

@backend *cmd:
    cd {{backend}}; just {{cmd}}

@frontend *cmd:
    cd {{frontend}}; just {{cmd}}

format:
    just backend format
    just frontend format

check:
    just backend check
    just frontend check

build:
    just frontend build
    just backend build

setup:
    just frontend setup
    just backend setup

fix:
    just backend fix
    just frontend fix

generate-code:
    just backend generate-openapi-spec
    just frontend generate-client
    rm api-docs.json
