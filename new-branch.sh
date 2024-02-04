#! /bin/bash

# for debug mode, set $execute to anything other than "true"
execute=true

echo "moving to new branch: $1"

if [[ "${execute}" == "true" ]]; then
  git checkout 52a42a9
  git switch -c $1
fi

read -d '' cargo_toml << EOF
[package]
name = "daily_bevy"
version = "0.1.0"
edition = "2021"

[dependencies]
bevy = "0.12.1"
EOF

if [[ "${execute}" == "true" ]]; then
  echo "$cargo_toml" > Cargo.toml
else
  echo "----- Cargo.toml -----"
  echo "$cargo_toml"
  echo "----------------------"
fi

read -d '' main_rs << EOF
// source: https://github.com/bevyengine/bevy/blob/v0.12.1/examples/$1.rs

use bevy::prelude::*;

fn main() {
    App::new().add_systems(Update, hello_world_system).run();
}

fn hello_world_system() {
    println!("hello world");
}
EOF

if [[ "${execute}" == "true" ]]; then
  mkdir src
  echo "$main_rs" > src/main.rs
else
  echo "----- src/main.rs -----"
  echo "$main_rs"
  echo "-----------------------"
fi
