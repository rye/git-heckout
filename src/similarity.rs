pub fn sublime(pattern: &str, string: &str) -> isize {
	sublime_fuzzy::best_match(pattern, string).map_or(0, |result| result.score())
}

#[cfg(test)]
mod tests {
	#[test]
	fn sublime_foobar_f() {
		assert_eq!(super::sublime("f", "foobar"), 83)
	}

	#[test]
	fn sublime_master_ma() {
		assert_eq!(super::sublime("ma", "master"), 117)
	}

	#[test]
	fn sublime_master_master() {
		assert_eq!(super::sublime("master", "master"), 896)
	}
}
