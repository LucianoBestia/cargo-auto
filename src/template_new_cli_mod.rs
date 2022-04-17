//! this strings are copied from the template_new_cli folder
//! because when publishing to crates.io, only the main bin-executable is transferred

pub fn copy_to_files(project_name: &str) {
    let folder_path = std::path::Path::new(project_name);
    std::fs::create_dir(folder_path).unwrap();
    for file_item in get_vec_file() {
        // rename/replace the project_name
        let file_name = file_item
            .file_name
            .replace("bestia_dev_cargo_auto_new_cli", project_name);
        let file_content = file_item
            .file_content
            .replace("bestia_dev_cargo_auto_new_cli", project_name);

        std::fs::write(folder_path.join(file_name), file_content.as_bytes()).unwrap();
    }
}

pub fn get_vec_file() -> Vec<crate::FileItem> {
    let mut vec_file = vec![];

    // region: files copied into strings by automation tasks
    vec_file.push(crate::FileItem {
        file_name: "Cargo.toml",
        file_content: r###"[package]
name = "bestia_dev_cargo_auto_new_cli"
version = "0.1.32"
description = "Basic Rust project template for CLI, more than just `cargo new hello`"
authors = ["bestia.dev <info@bestia.dev>"]
edition = "2021"
license = "MIT"
readme = "README.md"
repository = "https://github.com/bestia-dev/bestia_dev_cargo_auto_new_cli"
categories = ["rust-patterns"]
keywords = ["rust cli project template"]
publish = false

[dependencies]
log = "0.4"
pretty_env_logger="0.4.0"
thiserror = "1.0.30"
anyhow="1.0.56""###,
    });
    vec_file.push(crate::FileItem {
        file_name: ".gitignore",
        file_content: r###"
/target
**/*.rs.bk
Cargo.lock
"###,
    });
    vec_file.push(crate::FileItem {
        file_name: "src/utils_mod.rs",
        file_content: r###"// bestia_dev_cargo_auto_new_cli/src/utils_mod.rs

//! Just an example how to create and call a module in a separate file.
//!
//! This doc-comments will be compiled into the `docs`.

/// return uppercase
pub fn make_uppercase(my_name: &str) -> String {
    // return
    my_name.to_uppercase()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_make_uppercase() {
        assert_eq!(make_uppercase("abcd"), "ABCD");
        assert_eq!(make_uppercase("1234abcd"), "1234ABCD");
        assert_eq!(make_uppercase("čšž"), "ČŠŽ");
    }
}
"###,
    });
    vec_file.push(crate::FileItem {
        file_name: "src/bin/bestia_dev_cargo_auto_new_cli.rs",
        file_content: r###"//! bestia_dev_cargo_auto_new_cli/src/bin/bestia_dev_cargo_auto_new_cli.rs

// The `bin` has all the stdin and stdout.
// The `lib` must be in/out agnostic. That is the responsibility of the `bin`

// The `bin` uses the `anyhow` error library, the `lib` uses the `thiserror` library

/// entry point into the bin executable
fn main() {
    // logging is essential for every project
    pretty_env_logger::init();

    // super simple argument parsing. There are crates that can parse complex arguments.
    match std::env::args().nth(1).as_deref() {
        None | Some("--help") | Some("-h") => print_help(),
        Some("print") => match std::env::args().nth(2).as_deref() {
            // second argument
            Some(my_name) => {
                print_my_name(my_name);
            }
            None => println!("Missing arguments `my_name`."),
        },
        Some("upper") => match std::env::args().nth(2).as_deref() {
            // second argument
            Some(my_name) => {
                // this can return an error. Here is the last place I can deal with the error.
                match upper_my_name(my_name) {
                    // do nothing
                    Ok(()) => (),
                    // log error from anyhow
                    Err(err) => log::error!("{}", err),
                }
            }
            None => println!("Missing arguments `my_name`."),
        },
        _ => println!("Unrecognized arguments. Try `bestia_dev_cargo_auto_new_cli --help`"),
    }
}

/// print help
fn print_help() {
    println!(
        r#"
bestia_dev_cargo_auto_new_cli --help
bestia_dev_cargo_auto_new_cli print my_name
bestia_dev_cargo_auto_new_cli upper my_name
"#
    );
}

/// print my name
fn print_my_name(my_name: &str) {
    // call the function from the `lib`
    println!("{}", bestia_dev_cargo_auto_new_cli::format_hello_phrase(my_name));
}

/// print my name upper, can return error
fn upper_my_name(my_name: &str) -> anyhow::Result<()> {
    // the function from `lib`, can return error
    // use the ? syntax to bubble the error up one level or continue (early return)
    let upper = bestia_dev_cargo_auto_new_cli::format_upper_hello_phrase(my_name)?;
    println!("{}", upper);
    // return
    Ok(())
}
"###,
    });
    vec_file.push(crate::FileItem{
            file_name :"src/lib.rs",
            file_content : r###"// bestia_dev_cargo_auto_new_cli/src/lib.rs

// region: auto_md_to_doc_comments include README.md A //!
//! # bestia_dev_cargo_auto_new_cli
//!
//! **Basic Rust project template for CLI, more than just `cargo new hello`**  
//! ***version: 0.1.32 date: 2022-04-14 author: [bestia.dev](bestia.dev) repository: [Github](https://github.com/bestia-dev/bestia_dev_cargo_auto_new_cli)***  
//!
//! [![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-82-green.svg)](https://github.com/bestia-dev/bestia_dev_cargo_auto_new_cli/)
//! [![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-230-blue.svg)](https://github.com/bestia-dev/bestia_dev_cargo_auto_new_cli/)
//! [![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-25-purple.svg)](https://github.com/bestia-dev/bestia_dev_cargo_auto_new_cli/)
//! [![Lines in examples](https://img.shields.io/badge/Lines_in_examples-18-yellow.svg)](https://github.com/bestia-dev/bestia_dev_cargo_auto_new_cli/)
//! [![Lines in tests](https://img.shields.io/badge/Lines_in_tests-33-orange.svg)](https://github.com/bestia-dev/bestia_dev_cargo_auto_new_cli/)
//!
//! [![Licence](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/bestia-dev/bestia_dev_cargo_auto_new_cli/blob/main/LICENSE) [![Rust](https://github.com/bestia-dev/bestia_dev_cargo_auto_new_cli/workflows/RustAction/badge.svg)](https://github.com/bestia-dev/bestia_dev_cargo_auto_new_cli/)
//!
//! ## Motivation
//!
//! My first line I typed when I learned the Rust language was `cargo new hello`. It is extraordinary for learning Rust, but it is a rudimentary example, not really useful in practical life.
//!
//! I created this project template `bestia_dev_cargo_auto_new_cli` for a simple CLI application that has all the moving parts for a real life project.
//!
//! ## Separate bin and lib
//!
//! It is always good to split the project between a `bin` (executable) and a `lib` (library crate).
//!
//! Even for the smallest project. Maybe some other program will use the library eventually.
//!
//! All the input/output is coded in the `bin`. The library must not operate directly with the stdin/stdout, because some other caller of the library can have other ideas around input-output options.
//!
//! A separate `lib` enables to make good tests without worrying about input-output.
//!
//! ## super simple argument parsing
//!
//! I use a super simple code to parse CLI arguments inside the `src/bin/bestia_dev_cargo_auto_new_cli.rs`. There are crate libraries that enables very complex argument parsing if needed.
//!
//! ## automation_tasks_rs
//!
//! Building a project is always more complex then just `cargo build` and `cargo run`. There are always some files to copy or some content to copy from file to file. For this I use `cargo-auto` - automation tasks written in Rust language for the build process of rust projects.
//!
//! All the source is inside the folder `automation_tasks_rs`. It is pure Rust, it is easy to understand and modify to your needs.
//!
//! To start using it just type in `VSCode terminal`:
//!
//! ```bash
//! cargo auto
//! ```
//!
//! ```bash
//! User defined tasks in automation_tasks_rs:
//! cargo auto build - builds the crate in debug mode, fmt
//! cargo auto release - builds the crate in release mode, version from date, fmt, strip
//! cargo auto doc - builds the docs, copy to docs directory
//! cargo auto commit_and_push - commits with message and push with mandatory message
//!  if you use SSH, it is easy to start the ssh-agent in the background and ssh-add your credentials for git
//! cargo auto publish_to_crates_io - publish to crates.io, git tag
//! ```
//!
//! The `bash auto-completion` should work. If you type `cargo auto b` and press `tab` it should auto-complete to `build`. Look at the project <https://github.com/bestia-dev/dev_bestia_cargo_completion>.
//!
//! ```bash
//! cargo auto build
//! ```
//!
//! ```bash
//! Running automation task: build
//! old version: "0.1.18"
//! new version: '0.1.19'
//! $ cargo fmt
//! $ cargo build
//! Compiling bestia_dev_cargo_auto_new_cli v0.1.19 (/home/rustdevuser/rustprojects/bestia_dev_cargo_auto_new_cli)
//! Finished dev [unoptimized + debuginfo] target(s) in 2.72s
//!
//! After `cargo auto build`, run the compiled binary
//! run `./target/debug/bestia_dev_cargo_auto_new_cli print my_name`
//! later
//! run `cargo auto release`
//! ```
//!
//! After the task there is a recommendation what to do next.
//!
//! ```bash
//! cargo auto release
//! ```
//!
//! ```bash
//! Running automation task: release
//! old version: "0.1.20"
//! new version: '0.1.21'
//! new text: '
//! **Basic Rust project template for CLI, more than just `cargo new hello`**
//! ***version: 0.1.21 date: 2022-04-01 author: [bestia.dev](bestia.dev) repository: [Github](https://github.com/bestia-dev/bestia_dev_cargo_auto_new_cli)***'
//!
//! include_into_readme_md write file: README.md
//! $ cargo fmt
//! $ cargo build --release
//! Compiling bestia_dev_cargo_auto_new_cli v0.1.21 (/home/rustdevuser/rustprojects/bestia_dev_cargo_auto_new_cli)
//! Finished release [optimized] target(s) in 1.05s
//!
//! After `cargo auto release`, , run the compiled binary
//! run `./target/release/bestia_dev_cargo_auto_new_cli print my_name`
//! later
//! run `cargo auto doc`
//!
//! ```
//!
//! Release is incrementing the version number and date, copying the title and description from Cargo.toml to README.md. Calculates the lines of code in the project and makes badges from it in README.md. Copying the README into doc comments, so the documentation can be compiled later.
//!
//! ```bash
//! cargo auto doc
//! ```
//!
//! ```bash
//! Running automation task: doc
//! $ cargo doc --no-deps --document-private-items
//!  Documenting bestia_dev_cargo_auto_new_cli v0.1.21 (/home/rustdevuser/rustprojects/bestia_dev_cargo_auto_new_cli)
//! Finished dev [unoptimized + debuginfo] target(s) in 0.54s
//! $ rsync -a --info=progress2 --delete-after target/doc/ docs/
//! 2,787,371 100% 46.60MB/s 0:00:00 (xfr#56, to-chk=0/61) 
//!
//! After `cargo auto doc`, check `docs/index.html`. If ok, then 
//! run `cargo auto commit_and_push` with mandatory commit message
//! ```
//!
//! If you Ctrl+Click on the link `docs/index.html` it will open the file in VSCode editor. In the right corner you can click to see the Live Preview. It will open the preview for the html file in an integrated browser in VSCode. Very useful.
//! Now is a good time to run all the test before committing.
//!
//! ```bash
//! cargo test
//! ```
//!
//! If we are happy with the changes, we commit and push:
//!
//! ```bash
//! cargo auto commit_and_push "my message for commit"
//! ```
//!
//! ```bash
//! Running automation task: commit_and_push
//! $ git add -A && git commit -m "readme"
//! [main 3bdcc91] readme
//!  9 files changed, 443 insertions(+), 89 deletions(-)
//! $ git push
//! Enumerating objects: 36, done.
//! Counting objects: 100% (36/36), done.
//! Delta compression using up to 6 threads
//! Compressing objects: 100% (16/16), done.
//! Writing objects: 100% (19/19), 6.27 KiB | 1.25 MiB/s, done.
//! Total 19 (delta 11), reused 0 (delta 0), pack-reused 0
//! remote: Resolving deltas: 100% (11/11), completed with 10 local objects.
//! To https://github.com/bestia-dev/bestia_dev_cargo_auto_new_cli.git
//!  d0f31d3..3bdcc91 main -> main
//!
//! After `cargo auto commit and push`
//! run `cargo auto publish_to_crates_io`
//! ```
//!
//! And finally if you want to publish it on crates.io. First you need the `access token` you get from crates.io.
//!
//! ```bash
//! cargo login
//! # type the access token
//! cargo auto publish_to_crates_io
//! ```
//!
//! ## lib.rs doc-comments
//!
//! The entire README.md is copied into lib.rs. This can be annoying to watch. You can collapse the entire section clicking on `// region: auto_md_to_doc_comments include README.md`.
//!
//! You can use `// region:` and `// endregion:` to mark sections you want to collapse in the editor.
//!
//! From this doc-comments the `docs` will be created. Take a look and try to write what other users would want to read in the `docs`.
//!
//! ## Modules
//!
//! I added one module `utils_mod.rs` just to showcase how modules are used in separate files.
//!
//! ## Markdown
//!
//! README.md and all the doc-comments are in markdown. To separate paragraphs in markdown use an empty line between them.
//! I tried other variants like double-space or backslash, but an empty line is the most used in the wild.
//!
//! ## tests
//!
//! I added a unit-test, just to show how it looks. And an integration-test. So it is "ready-to-go".
//! Run them with `cargo test`.
//!
//! ## examples
//!
//! In the directory `examples` every rs file is a bin executable.
//! Run it with:
//!
//! ```bash
//! cargo run --example example_1
//! ```
//!
//! ## Error handling thiserror and anyhow
//!
//! The rule number one is never use `.unwrap()` in your real Rust code. It is a sign, you are not Error handling properly.
//! Maybe `unwrap()` can be fine for some fast learning examples, but for any real-life Rust code you must use some `Error handling`. There are many different ways to do that in Rust. I choose the pair of libraries `thiserror` and `anyhow`. The first is made for libraries, the second is made for bin-executables.  
//! The library needs an Enum with all the possible errors that this library can return. With `#[derive(Error)]` this enum get everything needed to be a true Rust error struct. Every error can have a formatting string and a struct of data.  
//! The bin-executable does not want to be involved in every possible error separately. It needs an umbrella for all possible errors with `anyhow::Result`.  
//! Inside the code, mostly propagate the errors with the `?` Operator after the `Result` value instead of unwrap() or the match expression.
//! In the tests we don't want to work with Error handling. There, instead of `.unwrap()`, use the similar function `.expect(&str)` that has an additional description string.
//!
//! ## cargo crev reviews and advisory
//!
//! We live in times of danger with [supply chain attacks](https://en.wikipedia.org/wiki/Supply_chain_attack).
//!
//! It is recommended to always use [cargo-crev](https://github.com/crev-dev/cargo-crev) to verify the trustworthiness of each of your dependencies.
//!
//! Please, spread this info.
//!
//! You can also read crev reviews quickly on the web:
//!
//! <https://web.crev.dev/rust-reviews/crates/>
//!
//! ## open-source and free as a beer
//!
//! My open-source projects are free as a beer (MIT license).
//!
//! I just love programming.
//!
//! But I need also to drink. If you find my projects and tutorials helpful,please buy me a beer donating on my [paypal](https://paypal.me/LucianoBestia).
//!
//! You know the price of a beer in your local bar ;-) So I can drink a free beer for your health :-)
//!
//! [Na zdravje!](https://translate.google.com/?hl=en&sl=sl&tl=en&text=Na%20zdravje&op=translate) [Alla salute!](https://dictionary.cambridge.org/dictionary/italian-english/alla-salute) [Prost!](https://dictionary.cambridge.org/dictionary/german-english/prost) [Nazdravlje!](https://matadornetwork.com/nights/how-to-say-cheers-in-50-languages/) 🍻
//!
// endregion: auto_md_to_doc_comments include README.md A //!

// The `bin` has all the stdin and stdout.
// The `lib` must be in/out agnostic. That is the responsibility of the `bin`

mod utils_mod;

// The `bin` uses the `anyhow` error library, the `lib` uses the `thiserror` library
use thiserror::Error;

/// all possible library errors for `thiserror`
#[derive(Error, Debug)]
pub enum LibraryError {
    #[error("name `{0}` is already uppercase")]
    Uppercase(String),
    #[error("unknown error")]
    Unknown,
}

/// format the hello phrase
pub fn format_hello_phrase(my_name: &str) -> String {
    log::info!("start format_hello_phrase()");
    // return
    format!("Hello {}!", my_name)
}

/// format the hello phrase with uppercase name
/// if it is already uppercase, return error with thiserror
pub fn format_upper_hello_phrase(my_name: &str) -> Result<String, LibraryError> {
    log::info!("start format_upper_hello_phrase()");
    // shadowing the same variable name:
    let upper_my_name = utils_mod::make_uppercase(my_name);
    if upper_my_name == my_name {
        return Err(LibraryError::Uppercase(my_name.to_string()));
    }

    // return
    Ok(format!("Hello {}!", &upper_my_name))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_format_upper_hello_phrase() {
        assert_eq!(format_upper_hello_phrase("abcd").expect("error"), "Hello ABCD!");
        assert!(format_upper_hello_phrase("ABCD").is_err());
    }
}
"###,
});
    vec_file.push(crate::FileItem{
            file_name :"README.md",
            file_content : r###"[comment]: # (auto_md_to_doc_comments segment start A)

# bestia_dev_cargo_auto_new_cli

[comment]: # (auto_cargo_toml_to_md start)

**Basic Rust project template for CLI, more than just `cargo new hello`**  
***version: 0.1.32 date: 2022-04-14 author: [bestia.dev](bestia.dev) repository: [Github](https://github.com/bestia-dev/bestia_dev_cargo_auto_new_cli)***  

[comment]: # (auto_cargo_toml_to_md end)

[comment]: # (auto_lines_of_code start)
[![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-82-green.svg)](https://github.com/bestia-dev/bestia_dev_cargo_auto_new_cli/)
[![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-230-blue.svg)](https://github.com/bestia-dev/bestia_dev_cargo_auto_new_cli/)
[![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-25-purple.svg)](https://github.com/bestia-dev/bestia_dev_cargo_auto_new_cli/)
[![Lines in examples](https://img.shields.io/badge/Lines_in_examples-18-yellow.svg)](https://github.com/bestia-dev/bestia_dev_cargo_auto_new_cli/)
[![Lines in tests](https://img.shields.io/badge/Lines_in_tests-33-orange.svg)](https://github.com/bestia-dev/bestia_dev_cargo_auto_new_cli/)

[comment]: # (auto_lines_of_code end)

[![Licence](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/bestia-dev/bestia_dev_cargo_auto_new_cli/blob/main/LICENSE) [![Rust](https://github.com/bestia-dev/bestia_dev_cargo_auto_new_cli/workflows/RustAction/badge.svg)](https://github.com/bestia-dev/bestia_dev_cargo_auto_new_cli/)

## Edit this README.md file

Edit this README file with your data. But leave the markers: auto_md_to_doc_comments, auto_lines_of_code, auto_cargo_toml_to_md and similar, because the automation tasks need them.  
Modify the title and description only in Cargo.toml. Automation tasks will copy that into README.md.  
Lines of code are filled automatically from automation tasks.  
Find `bestia.dev` everywhere and change it with your username.

## Motivation

My first line I typed when I learned the Rust language was `cargo new hello`. It is extraordinary for learning Rust, but it is a rudimentary example, not really useful in practical life.

I created this project template `bestia_dev_cargo_auto_new_cli` for a simple CLI application that has all the moving parts for a real life project.

## Separate bin and lib

It is always good to split the project between a `bin` (executable) and a `lib` (library crate).

Even for the smallest project. Maybe some other program will use the library eventually.

All the input/output is coded in the `bin`: keyboard and monitor (stdin and stdout), access to files and some access to network.  
The library must not operate directly with the stdin/stdout, because some other caller of the library can have other ideas around input-output options. Maybe it is a Graphical user interface that does thing completely different than CLI applications.

A separate `lib` enables to make good tests and examples without worrying about input-output.

## super simple argument parsing

I use a super simple code to parse CLI arguments inside the `src/bin/bestia_dev_cargo_auto_new_cli.rs`. There are crate libraries that enables very complex argument parsing if needed.

## automation_tasks_rs

Building a project is always more complex then just `cargo build` and `cargo run`. There are always some files to copy or some content to copy from file to file. For this I use `cargo-auto` - automation tasks written in Rust language for the build process of rust projects.

All the source is inside the folder `automation_tasks_rs`. It is pure Rust, it is easy to understand and modify to your needs.

To start using it just type in `VSCode terminal`:

```bash
cargo auto
```

```bash
User defined tasks in automation_tasks_rs:
cargo auto build - builds the crate in debug mode, fmt
cargo auto release - builds the crate in release mode, version from date, fmt, strip
cargo auto doc - builds the docs, copy to docs directory
cargo auto commit_and_push - commits with message and push with mandatory message
 if you use SSH, it is easy to start the ssh-agent in the background and ssh-add your credentials for git
cargo auto publish_to_crates_io - publish to crates.io, git tag
```

The `bash auto-completion` should work. If you type `cargo auto b` and press `tab` it should auto-complete to `build`. Look at the project <https://github.com/bestia-dev/dev_bestia_cargo_completion>.

```bash
cargo auto build
```

```bash
Running automation task: build
old version: "0.1.18"
new version: '0.1.19'
$ cargo fmt
$ cargo build
Compiling bestia_dev_cargo_auto_new_cli v0.1.19 (/home/rustdevuser/rustprojects/bestia_dev_cargo_auto_new_cli)
Finished dev [unoptimized + debuginfo] target(s) in 2.72s

After `cargo auto build`, run the compiled binary
run `./target/debug/bestia_dev_cargo_auto_new_cli print my_name`
later
run `cargo auto release`
```

After the task there is a recommendation what to do next.

```bash
cargo auto release
```

```bash
Running automation task: release
old version: "0.1.20"
new version: '0.1.21'
new text: '
**Basic Rust project template for CLI, more than just `cargo new hello`**
***version: 0.1.21 date: 2022-04-01 author: [bestia.dev](bestia.dev) repository: [Github](https://github.com/bestia-dev/bestia_dev_cargo_auto_new_cli)***'

include_into_readme_md write file: README.md
$ cargo fmt
$ cargo build --release
Compiling bestia_dev_cargo_auto_new_cli v0.1.21 (/home/rustdevuser/rustprojects/bestia_dev_cargo_auto_new_cli)
Finished release [optimized] target(s) in 1.05s

After `cargo auto release`, , run the compiled binary
run `./target/release/bestia_dev_cargo_auto_new_cli print my_name`
later
run `cargo auto doc`

```

Release is incrementing the version number and date, copying the title and description from Cargo.toml to README.md. Calculates the lines of code in the project and makes badges from it in README.md. Copying the README into doc comments, so the documentation can be compiled later.

```bash
cargo auto doc
```

```bash
Running automation task: doc
$ cargo doc --no-deps --document-private-items
 Documenting bestia_dev_cargo_auto_new_cli v0.1.21 (/home/rustdevuser/rustprojects/bestia_dev_cargo_auto_new_cli)
Finished dev [unoptimized + debuginfo] target(s) in 0.54s
$ rsync -a --info=progress2 --delete-after target/doc/ docs/
2,787,371 100% 46.60MB/s 0:00:00 (xfr#56, to-chk=0/61) 

After `cargo auto doc`, check `docs/index.html`. If ok, then 
run `cargo auto commit_and_push` with mandatory commit message
```

If you Ctrl+Click on the link `docs/index.html` it will open the file in VSCode editor. In the right corner you can click to see the Live Preview. It will open the preview for the html file in an integrated browser in VSCode. Very useful.
Now is a good time to run all the test before committing.

```bash
cargo test
```

If we are happy with the changes, we commit and push:

```bash
cargo auto commit_and_push "my message for commit"
```

```bash
Running automation task: commit_and_push
$ git add -A && git commit -m "readme"
[main 3bdcc91] readme
 9 files changed, 443 insertions(+), 89 deletions(-)
$ git push
Enumerating objects: 36, done.
Counting objects: 100% (36/36), done.
Delta compression using up to 6 threads
Compressing objects: 100% (16/16), done.
Writing objects: 100% (19/19), 6.27 KiB | 1.25 MiB/s, done.
Total 19 (delta 11), reused 0 (delta 0), pack-reused 0
remote: Resolving deltas: 100% (11/11), completed with 10 local objects.
To https://github.com/bestia-dev/bestia_dev_cargo_auto_new_cli.git
 d0f31d3..3bdcc91 main -> main

After `cargo auto commit and push`
run `cargo auto publish_to_crates_io`
```

And finally if you want to publish it on crates.io. First you need the `access token` you get from crates.io.

```bash
cargo login
# type the access token
cargo auto publish_to_crates_io
```

## lib.rs doc-comments

The entire README.md is copied into lib.rs. This can be annoying to watch. You can collapse the entire section clicking on `// region: auto_md_to_doc_comments include README.md`.

You can use `// region:` and `// endregion:` to mark sections you want to collapse in the editor.

From this doc-comments the `docs` will be created. Take a look and try to write what other users would want to read in the `docs`.

## Modules

I added one module `utils_mod.rs` just to showcase how modules are used in separate files.

## Markdown

README.md and all the doc-comments are in markdown. To separate paragraphs in markdown use an empty line between them.
I tried other variants like double-space or backslash, but an empty line is the most used in the wild.

## tests

I added a unit-test, just to show how it looks. And an integration-test. So it is "ready-to-go".
Run them with `cargo test`.

## examples

In the directory `examples` every rs file is a bin executable.
Run it with:

```bash
cargo run --example example_1
```

## Error handling thiserror and anyhow

The rule number one is never use `.unwrap()` in your real Rust code. It is a sign, you are not Error handling properly.
Maybe `unwrap()` can be fine for some fast learning examples, but for any real-life Rust code you must use some `Error handling`. There are many different ways to do that in Rust. I choose the pair of libraries `thiserror` and `anyhow`. The first is made for libraries, the second is made for bin-executables.  
The library needs an Enum with all the possible errors that this library can return. With `#[derive(Error)]` this enum get everything needed to be a true Rust error struct. Every error can have a formatting string and a struct of data.  
The bin-executable does not want to be involved in every possible error separately. It needs an umbrella for all possible errors with `anyhow::Result`.  
Inside the code, mostly propagate the errors with the `?` Operator after the `Result` value instead of unwrap() or the match expression.
In the tests we don't want to work with Error handling. There, instead of `.unwrap()`, use the similar function `.expect(&str)` that has an additional description string.

## cargo crev reviews and advisory

We live in times of danger with [supply chain attacks](https://en.wikipedia.org/wiki/Supply_chain_attack).

It is recommended to always use [cargo-crev](https://github.com/crev-dev/cargo-crev) to verify the trustworthiness of each of your dependencies.

Please, spread this info.

You can also read crev reviews quickly on the web:

<https://web.crev.dev/rust-reviews/crates/>

## open-source and free as a beer

My open-source projects are free as a beer (MIT license).

I just love programming.

But I need also to drink. If you find my projects and tutorials helpful,please buy me a beer donating on my [paypal](https://paypal.me/LucianoBestia).

You know the price of a beer in your local bar ;-) So I can drink a free beer for your health :-)

[Na zdravje!](https://translate.google.com/?hl=en&sl=sl&tl=en&text=Na%20zdravje&op=translate) [Alla salute!](https://dictionary.cambridge.org/dictionary/italian-english/alla-salute) [Prost!](https://dictionary.cambridge.org/dictionary/german-english/prost) [Nazdravlje!](https://matadornetwork.com/nights/how-to-say-cheers-in-50-languages/) 🍻

[comment]: # (auto_md_to_doc_comments segment end A)
"###,
});
    vec_file.push(crate::FileItem {
        file_name: "LICENSE",
        file_content: r###"MIT License

Copyright (c) 2022 bestia.dev

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
"###,
    });
    vec_file.push(crate::FileItem {
        file_name: "tests/integration_test.rs",
        file_content: r###"// tests/integration_test.rs

use bestia_dev_cargo_auto_new_cli::*;

#[test]
fn integration_test_01() {
    assert_eq!(format_hello_phrase("abcd"), "Hello abcd!");
    assert_eq!(format_upper_hello_phrase("abcd").expect("error"), "Hello ABCD!");
}

#[test]
fn integration_test_02_error_check() {
    assert!(format_upper_hello_phrase("ABCD").is_err());
}
"###,
    });
    vec_file.push(crate::FileItem {
        file_name: "examples/example_1.rs",
        file_content: r###"// examples/example_1.rs

//! A simple example how to use the `lib`

use bestia_dev_cargo_auto_new_cli::*;

/// example how to use format_hello_phrase() and format_upper_hello_phrase()
fn main() {
    let my_name = "john doe";
    let phrase = format_hello_phrase(my_name);
    println!("{}", phrase);

    // possible error must be processed
    match format_upper_hello_phrase(my_name) {
        Ok(phrase) => println!("{}", phrase),
        Err(err) => log::error!("Error: {}", err),
    }
}
"###,
    });
    vec_file.push(crate::FileItem {
        file_name: "automation_tasks_rs/Cargo.toml",
        file_content: r###"
[package]
name = "automation_tasks_rs"
version = "0.1.1"
authors = ["bestia.dev <info@bestia.dev>"]
edition = "2018"
description = "cargo auto - automation tasks written in Rust language"
publish = false

[dependencies]
cargo_auto_lib = "0.7.24""###,
    });
    vec_file.push(crate::FileItem {
        file_name: "automation_tasks_rs/.gitignore",
        file_content: r###"/target
    "###,
    });
    vec_file.push(crate::FileItem{
            file_name :"automation_tasks_rs/src/main.rs",
            file_content : r###"//! automation_tasks_rs for bestia_dev_cargo_auto_new_cli

use cargo_auto_lib::*;

fn main() {
    exit_if_not_run_in_rust_project_root_directory();

    // get CLI arguments
    let mut args = std::env::args();
    // the zero argument is the name of the program
    let _arg_0 = args.next();
    match_arguments_and_call_tasks(args);
}

// region: match, help and completion

/// match arguments and call tasks functions
fn match_arguments_and_call_tasks(mut args: std::env::Args) {
    // the first argument is the user defined task: (no argument for help), build, release,...
    let arg_1 = args.next();
    match arg_1 {
        None => print_help(),
        Some(task) => {
            if &task == "completion" {
                completion();
            } else {
                println!("Running automation task: {}", &task);
                if &task == "build" {
                    task_build();
                } else if &task == "release" {
                    task_release();
                } else if &task == "test" {
                    task_test();
                } else if &task == "doc" {
                    task_doc();
                } else if &task == "commit_and_push" {
                    let arg_2 = args.next();
                    task_commit_and_push(arg_2);
                //} else if &task == "publish_to_crates_io" {
                //    task_publish_to_crates_io();
                } else {
                    println!("Task {} is unknown.", &task);
                    print_help();
                }
            }
        }
    }
}

/// write a comprehensible help for user defined tasks
fn print_help() {
    println!(
        r#"
User defined tasks in automation_tasks_rs:
cargo auto build - builds the crate in debug mode, fmt, increment version
cargo auto release - builds the crate in release mode, fmt, increment version
cargo auto doc - builds the docs, copy to docs directory
cargo auto test - runs all the tests
cargo auto commit_and_push "message" - commits with message and push with mandatory message
      (If you use SSH, it is easy to start the ssh-agent in the background and ssh-add your credentials for git.)
"#
// cargo auto publish_to_crates_io - publish to crates.io, git tag
//      (You need to save the credentials before publishing. On crates.io get the 'access token'. Then save it locally with the command ` cargo login TOKEN`)
    );
}

/// sub-command for bash auto-completion of `cargo auto` using the crate `dev_bestia_cargo_completion`
fn completion() {
    let args: Vec<String> = std::env::args().collect();
    let word_being_completed = args[2].as_str();
    let last_word = args[3].as_str();

    if last_word == "cargo-auto" || last_word == "auto" {
        let sub_commands = vec!["build", "release", "doc","test", "commit_and_push"];
        // , "publish_to_crates_io"
        completion_return_one_or_more_sub_commands(sub_commands, word_being_completed);
    }
    /*
    // the second level if needed
    else if last_word == "new" {
        let sub_commands = vec!["with_lib"];
        completion_return_one_or_more_sub_commands(sub_commands, word_being_completed);
    }
    */
}

// endregion: match, help and completion

// region: tasks

/// cargo build
fn task_build() {
    let cargo_toml = CargoToml::read();
    auto_version_increment_semver_or_date();
    run_shell_command("cargo fmt");
    run_shell_command("cargo build");
    println!(
        r#"
After `cargo auto build`, run the compiled binary
run `./target/debug/{package_name} print my_name`, if ok, then
run `./target/debug/{package_name} upper my_name`, if ok, then
run `./target/debug/{package_name} upper MY_NAME`, if ok, then
run `cargo auto release`
"#, 
package_name = cargo_toml.package_name(),
    );
}

/// cargo build --release
fn task_release() {
    let cargo_toml = CargoToml::read();
    auto_version_increment_semver_or_date();
    auto_cargo_toml_to_md();
    auto_lines_of_code("");

    run_shell_command("cargo fmt");
    run_shell_command("cargo build --release");
    println!(
        r#"
After `cargo auto release`, run the compiled binary
run `./target/release/{package_name} print my_name` if ok, then
run `./target/release/{package_name} upper my_name` if ok, then
run `./target/release/{package_name} upper MY_NAME` if ok, then
run `cargo auto doc`
"#,
package_name = cargo_toml.package_name(),
    );
}

/// cargo doc, then copies to /docs/ folder, because this is a github standard folder
fn task_doc() {
    let cargo_toml = CargoToml::read();
    auto_cargo_toml_to_md();
    auto_lines_of_code("");
    auto_md_to_doc_comments();

    run_shell_command("cargo doc --no-deps --document-private-items");
    // copy target/doc into docs/ because it is github standard
    run_shell_command("rsync -a --info=progress2 --delete-after target/doc/ docs/");
    // Create simple index.html file in docs directory
    run_shell_command(&format!("echo \"<meta http-equiv=\\\"refresh\\\" content=\\\"0; url={}/index.html\\\" />\" > docs/index.html",cargo_toml.package_name().replace("-","_")));    
    // message to help user with next move
    println!(
        r#"
After `cargo auto doc`, check `docs/index.html`. If ok, then 
run `cargo auto test`
"#
    );
}

/// cargo test
fn task_test() {
    run_shell_command("cargo test");
    println!(
        r#"
After `cargo auto test`. If ok, then 
run `cargo auto commit_and_push "message"` with mandatory commit message
"#
    );
}

/// commit and push
fn task_commit_and_push(arg_2: Option<String>) {
    match arg_2 {
        None => println!("Error: message for commit is mandatory"),
        Some(message) => {
            run_shell_command(&format!(r#"git add -A && git commit -m "{}""#, message));
            run_shell_command("git push");
            println!(
                r#"
After `cargo auto commit_and_push "message"`
run `cargo auto publish_to_crates_io`
"#
            );
        }
    }
}

/*
/// publish to crates.io and git tag
fn task_publish_to_crates_io() {
    println!(r#"The crates.io access token must already be saved locally with `cargo login TOKEN`"#);
    let cargo_toml = CargoToml::read();
    // git tag
    let shell_command = format!(
        "git tag -f -a v{version} -m version_{version}",
        version = cargo_toml.package_version()
    );
    run_shell_command(&shell_command);

    // cargo publish
    run_shell_command("cargo publish");
    println!(
        r#"
After `cargo auto publish_to_crates_io`, 
check `https://crates.io/crates/{package_name}`.
Add the dependency `{package_name} = "{package_version}"` to your rust project and check how it works.
"#,
        package_name = cargo_toml.package_name(),
        package_version = cargo_toml.package_version()
    );
}
*/

// endregion: tasks
"###,
});
    // endregion: files copied into strings by automation tasks

    // return
    vec_file
}