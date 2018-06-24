
fn main() {

    let distance = normalised_affine_gap_distance("radzikhovskyy", "jonathan");
    println!("{}", distance);
    for _ in 0..10000000 {
        normalised_affine_gap_distance("radzikhovskyy", "jonathan");
    }
}

fn min(a: f64, b: f64) -> f64 {
    if a < b {
        a
    } else {
        b
    }
}

fn affine_gap_distance<'a>(mut string1: &'a str, mut string2: &'a str) -> f64 {

	let match_weight = 1 as f64;
	let mismatch_weight = 11 as f64;
	let gap_weight = 10 as f64;
	let space_weight = 7 as f64;
	let abbreviation_scale = 0.125;

	let mut length1 = string1.len();
	let mut length2 = string2.len();

	if string1 == string2 &&
		match_weight == min(min(match_weight, mismatch_weight), gap_weight) {
		return match_weight as f64 * length1 as f64
	}

	if length1 < length2 {
        let tmp = string2;
        string2 = string1;
        string1 = tmp;
        let tmp = length2;
        length2 = length1;
        length1 = tmp;
	};

	let mut d = vec![0.0; length1 + 1];
	let mut v_current = vec![0.0; length1 + 1];
	let mut v_previous = vec![0.0; length1 + 1];

	for j in 1..(length1 + 1) {
		v_current[j] = gap_weight + space_weight*j as f64;
		d[j] = std::i32::MAX as f64;
	};

	for i in 1..(length2 + 1) {
		let char2 = string2.get(i-1..i).unwrap();

        for i in 0..length1 + 1 {
            v_previous[i] = v_current[i]
        }

		v_current[0] = (gap_weight + (space_weight*i as f64)) as f64;
		let mut i = std::i32::MAX as f64;

		for j in 1..(length1 + 1) {
			let char1 = string1.get(j-1..j).unwrap();

			if j <= length2 {
				i = min(i, v_current[j-1]+gap_weight) + space_weight;
			} else {
				i = min(i, v_current[j-1]+gap_weight *abbreviation_scale) + space_weight * abbreviation_scale;
			};
			d[j] = min(d[j], v_previous[j]+gap_weight) + space_weight;

			let mut m: f64;
			if char2 == char1 {
				m = v_previous[j-1] + match_weight;
			} else {
				m = v_previous[j-1] + mismatch_weight;
			};

			v_current[j] = min(min(i, d[j]), m)
		};
	};
	v_current[length1]
}
fn normalised_affine_gap_distance(string1: &str, string2: &str) -> f64 {

	let length1 = string1.len();
	let length2 = string2.len();

	let normalizer = length1 + length2;

	let distance = affine_gap_distance(string1, string2);

	distance / normalizer as f64
}
