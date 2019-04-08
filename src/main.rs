extern crate distance;
extern crate git2;

fn find_repository() -> Result<git2::Repository, git2::Error> {
	git2::Repository::open_from_env()
}

fn main() {
	let repository = find_repository().expect("Could not find a repository");

	println!("git-heckout: {:?}", repository.workdir().unwrap());

	println!("{}", distance::levenshtein("blep", "blop"));
}
