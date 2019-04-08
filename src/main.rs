extern crate distance;
extern crate git2;

use std::ffi::OsStr;
use std::process::{Command, ExitStatus, Stdio};

fn branch_names(repository: &git2::Repository) -> Result<Vec<String>, git2::Error> {
	let branches: git2::Branches = repository
		.branches(None)
		.expect("Unable to get branches on repository. Is something messed up?");

	branches
		.map(
			|branch_result: Result<(_, _), git2::Error>| -> Result<String, git2::Error> {
				let branch_tuple: (git2::Branch, _) = branch_result.expect("Branch has an issue.");
				let branch: git2::Branch = branch_tuple.0;
				let name: Result<Option<&str>, git2::Error> = branch.name();

				match name {
					Ok(name) => {
						let branch_name = name.expect("valid utf-8").to_string();

						Ok(branch_name)
					}
					Err(error) => Err(error),
				}
			},
		)
		.collect()
}

// TODO Accept an OpenMode that lets you pick how it finds it
fn find_repository() -> Result<git2::Repository, git2::Error> {
	git2::Repository::open_from_env()
}

fn checkout<T: AsRef<OsStr>>(branch: T) -> ExitStatus {
	Command::new("git")
		.arg("checkout")
		.arg(branch)
		.stdin(Stdio::inherit())
		.stdout(Stdio::inherit())
		.stderr(Stdio::inherit())
		.status()
		.expect("failed to spawn `git checkout` subprocess")
}

fn handle_exit_status_code(code: i32) -> i32 {
	if let 0 = code {
		// do nothing... exit was okay
	} else {
		eprintln!("git-checkout exited with status code {:?}", code)
	}

	code
}

fn similarity(str1: &str, str2: &str) -> f32 {
	#[allow(clippy::cast_precision_loss)]
	let distance = distance::levenshtein(str1, str2) as f32;
	#[allow(clippy::cast_precision_loss)]
	let biggest_len = std::cmp::max(str1.len(), str2.len()) as f32;

	distance / biggest_len
}

fn main() {
	let repository = find_repository().expect("Could not find a repository");

	let branch = std::env::args().last().expect("Expected a last argument");

	let mut branch_names: Vec<String> = branch_names(&repository).unwrap();

	if branch_names.contains(&branch) {
		if let Some(code) = checkout(branch).code() {
			std::process::exit(handle_exit_status_code(code))
		} else {
			eprintln!("git-checkout terminated by signal");
			std::process::exit(1)
		}
	} else {
	}
}
