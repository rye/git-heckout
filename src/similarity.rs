pub fn sublime(pattern: &str, string: &str) -> isize {
	sublime_fuzzy::best_match(pattern, string).map_or(0, |result| result.score())
}

#[cfg(test)]
mod tests {
	#[test]
	fn sublime_substrings_same_score() {
		// Sub-strings have the same score
		assert!(super::sublime("master", "m") == super::sublime("master", "ma"));
		assert!(super::sublime("master", "ma") == super::sublime("master", "mas"));
		assert!(super::sublime("master", "mas") == super::sublime("master", "mast"));
		assert!(super::sublime("master", "mast") == super::sublime("master", "maste"));
		// even across different comparison strings (transitively)
		assert!(super::sublime("masterz", "m") == super::sublime("master", "m"));
		assert!(super::sublime("masterz", "ma") == super::sublime("master", "ma"));
		assert!(super::sublime("masterz", "mas") == super::sublime("master", "mas"));
		assert!(super::sublime("masterz", "mast") == super::sublime("master", "mast"));
		assert!(super::sublime("masterz", "maste") == super::sublime("master", "maste"));
	}

	#[test]
	fn sublime_substrings_lower_than_equal() {
		// Sub-strings have a lower score than self-similarity
		assert!(super::sublime("master", "maste") < super::sublime("master", "master"));
	}

	#[test]
	fn sublime_substrings_less_than_self_similiarity_even_for_shorter() {
		// Self-similarity always trumps substring scores, even for short strings
		assert!(super::sublime("master", "m") < super::sublime("m", "m"));
		assert!(super::sublime("master", "ma") < super::sublime("ma", "ma"));
		assert!(super::sublime("master", "mas") < super::sublime("mas", "mas"));
		assert!(super::sublime("master", "mast") < super::sublime("mast", "mast"));
		assert!(super::sublime("master", "maste") < super::sublime("maste", "maste"));
	}

	#[test]
	fn sublime_self_similiarity_stronger_for_longer_strings() {
		// Self-similarity scores are higher for longer strings
		assert!(super::sublime("m", "m") < super::sublime("ma", "ma"));
		assert!(super::sublime("ma", "ma") < super::sublime("mas", "mas"));
		assert!(super::sublime("mas", "mas") < super::sublime("mast", "mast"));
		assert!(super::sublime("mast", "mast") < super::sublime("maste", "maste"));
		assert!(super::sublime("maste", "maste") < super::sublime("master", "master"));
		assert!(super::sublime("master", "master") < super::sublime("masterz", "masterz"));
	}
}
