use git_heckout::similarity;

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

fn checkout(branch: &str) -> std::io::Result<std::process::ExitStatus> {
	std::process::Command::new("git")
		.arg("checkout")
		.arg(branch)
		.spawn()
		.expect("failed to spawn git checkout")
		.wait()
}

fn main() {
	let repository = find_repository().expect("Could not find a repository");

	let branch = std::env::args().last().expect("Expected a last argument");

	let mut branch_names: Vec<String> = branch_names(&repository).unwrap();

	if branch_names.contains(&branch) {
		std::process::exit(match checkout(&branch) {
			Ok(_) => 0,
			Err(_) => 1,
		});
	} else {
		// Sort branch names in descending distance from provided branch argument
		// so that we can just use the first element instead of having to walk the
		// iterator to the end (linear in the number of branches)
		branch_names.sort_by(|a, b| {
			let a_dist = similarity::sublime(&branch, a);
			let b_dist = similarity::sublime(&branch, b);

			b_dist.partial_cmp(&a_dist).unwrap()
		});

		// Grab the highest similarity branch (at the front of the iterator chain)
		let best_match = &branch_names[0];

		std::process::exit(match checkout(&best_match) {
			Ok(_) => 0,
			Err(_) => 1,
		});
	}
}
