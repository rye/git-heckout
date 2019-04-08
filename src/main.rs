extern crate distance;
extern crate git2;

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

fn main() {
	let repository = find_repository().expect("Could not find a repository");

	let branch = std::env::args().last().expect("Expected a last argument");


}
