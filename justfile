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

build-server:
    cargo build --release -p clip-mash-server
    let triple = rustc -vV | parse --regex "host: (?<triple>.*)"
    cp target/release/clip-mash-server target/release/clip-mash-server

;; format:
;;     just server format
;;     just tauri format

;; check:
;;     just server check
;;     just tauri check

;; build:
;;     just tauri build
;;     just server build

;; setup:
;;     just tauri setup
;;     just server setup

;; fix:
;;     just server fix
;;     just tauri fix

;; generate-code:
;;     just server generate-openapi-spec
;;     just tauri generate-client
;;     rm api-docs.json
