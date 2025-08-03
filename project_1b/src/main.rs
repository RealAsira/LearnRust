/*
running `cargo new project_name` creates a new project in "project_name" sub-dir
included is Cargo.toml, a git repo including .gitignore, and source code in "src" sub-dir
the top level directory (project_name dir) is for README files, licensing, config, etc

if a project was created by writing a plain rust file, then use cargo init after putting the .rs into src sub-dir

check compilability using `cargo check`
to build using cargo run `cargo build` in "project_name" directory
run using `./target/debug/project_name`
to build and run simultaneously, use `cargo run`

to build a release version, use `cargo build --release` which will optimize and output an executable to ./target/release
*/


fn main() {
    println!("Goodnight sky!");
}
