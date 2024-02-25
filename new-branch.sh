#! /bin/bash

# for debug mode, set $execute to anything other than "true"
execute=true

BEVY_VERSION="0.13.0"

# usage: ./new-branch.sh path/to/example "Example Name"
# e.g.   ./new-branch.sh 2d/2d_gizmos "2D Gizmos"

echo "moving to new branch: $1"

if [[ "${execute}" == "true" ]]; then
  git -c advice.detachedHead=false checkout 52a42a9
  git switch -c $1
fi

N_BRANCH=$(git branch -r | wc -l | sed 's/ //g')

read -d '' cargo_toml << EOF
[package]
name = "daily_bevy"
version = "0.1.0"
edition = "2021"

[dependencies]
bevy = "$BEVY_VERSION"
EOF

if [[ "${execute}" == "true" ]]; then
  echo "$cargo_toml" > Cargo.toml
else
  echo "----- Cargo.toml -----"
  echo "$cargo_toml"
  echo "----------------------"
fi

read -d '' main_rs << EOF
// source: https://github.com/bevyengine/bevy/blob/v$BEVY_VERSION/examples/$1.rs

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

read -d '' readme << EOF
# daily-bevy

Learn [Bevy](https://bevyengine.org/) by exploring a small example (almost) every day.

[Bevy](https://github.com/bevyengine/bevy/) is a free, open-source, cross-platform (Windows, macOS, Linux, Web, iOS, Android) game engine written in [Rust](https://www.rust-lang.org/).

This README shows entry #$N_BRANCH in this series. All other entries can be found at [daily-bevy/branches](https://github.com/awwsmm/daily-bevy/branches).

## $2

Today is day #$N_BRANCH of Daily Bevy.

This kata uses [Bevy \`v$BEVY_VERSION\`](https://github.com/bevyengine/bevy/tree/v$BEVY_VERSION).

### Today's Kata

...

#### The Code

Here's the \`main.rs\` for this example

\`\`\`rust
...
\`\`\`

And here's the \`Cargo.toml\` for this example

\`\`\`toml
[dependencies]
bevy = "$BEVY_VERSION"
\`\`\`

#### Discussion

...

## Learn More

If you found this first kata interesting, head over to [daily-bevy/branches](https://github.com/awwsmm/daily-bevy/branches) to see the rest of them.

If you have questions, comments, or corrections, please head over to [daily-bevy/discussions](https://github.com/awwsmm/daily-bevy/discussions) to join the conversation.

If you like what you've read above, you can [follow me on Bluesky](https://bsky.app/profile/awwsmm.bsky.social) or [Mastodon](https://mas.to/@awwsmm).
EOF

if [[ "${execute}" == "true" ]]; then
  echo "$readme" > README.md
else
  echo "------ README.md ------"
  echo "$readme"
  echo "-----------------------"
fi
