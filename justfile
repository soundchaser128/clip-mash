set shell := ["nu", "-c"]

project := justfile_directory()
tauri := project + "/crates/clip-mash-app"
server := project + "/crates/clip-mash-server"

default:
  @just --list

@server *cmd:
    cd {{server}}; just {{cmd}}

@tauri *cmd:
    cd {{tauri}}; just {{cmd}}

format:
    @just tauri format
    cargo +nightly fmt