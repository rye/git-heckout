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

fn checkout(repository: &mut git2::Repository, branch: &str) -> Result<(), git2::Error> {
	let branch_ref_name: &str = &format!("refs/heads/{}", &branch);

	repository.set_head(branch_ref_name)?;
	repository.checkout_head(None)
}

fn similarity(str1: &str, str2: &str) -> f32 {
	#[allow(clippy::cast_precision_loss)]
	let distance = distance::levenshtein(str1, str2) as f32;
	#[allow(clippy::cast_precision_loss)]
	let biggest_len = std::cmp::max(str1.len(), str2.len()) as f32;

	distance / biggest_len
}

fn interpret_checkout_result(branch: &str, result: Result<(), git2::Error>) {
	match result {
		Ok(_) => {
			println!("Successfully checked out branch {}.", &branch);
		}
		Err(e) => {
			eprintln!("An error occurred while checking out {}: {}", &branch, e);
		}
	}
}

fn main() {
	let mut repository = find_repository().expect("Could not find a repository");

	let branch = std::env::args().last().expect("Expected a last argument");

	let mut branch_names: Vec<String> = branch_names(&repository).unwrap();

	if branch_names.contains(&branch) {
		let result = checkout(&mut repository, &branch);
		interpret_checkout_result(&branch, result);
	} else {
		branch_names.sort_by(|a, b| {
			let a_dist = similarity(&branch, a);
			let b_dist = similarity(&branch, b);

			a_dist.partial_cmp(&b_dist).unwrap()
		});

		let lowest_distance_branch = &branch_names[0];

		let result = checkout(&mut repository, &lowest_distance_branch);
		interpret_checkout_result(&lowest_distance_branch, result);
	}
}
