pub fn sublime(pattern: &str, string: &str) -> isize {
	sublime_fuzzy::best_match(pattern, string).map_or(0, |result| result.score()).into()
}

#[cfg(test)]
mod tests {
	#[bench]
	fn bench_sublime_foobar_f(b: &mut test::Bencher) {
		b.iter(|| assert_eq!(super::sublime("f", "foobar"), 83))
	}

	#[bench]
	fn bench_sublime_master_ma(b: &mut test::Bencher) {
		b.iter(|| assert_eq!(super::sublime("ma", "master"), 117))
	}

	#[bench]
	fn bench_sublime_master_master(b: &mut test::Bencher) {
		b.iter(|| assert_eq!(super::sublime("master", "master"), 896))
	}
}
