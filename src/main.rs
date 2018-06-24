
fn main() {
    for _ in 0..10000000 {
        normalised_affine_gap_distance("radzikhovskyy".to_string(), "jonathan".to_string());
    }
}


fn min(a: f64, b: f64) -> f64 {
    if a < b {
        a
    } else {
        b
    }
}

fn affine_gap_distance(string_a: String, string_b: String) -> f64 {

	let matchWeight = 1 as f64;
	let mismatchWeight = 11 as f64;
	let gapWeight = 10 as f64;
	let spaceWeight = 7 as f64;
	let abbreviation_scale = 0.125;

	let mut string1 = string_b;
	let mut string2 = string_a;

	let mut length1 = string1.len();
	let mut length2 = string2.len();

	if string1 == string2 &&
		matchWeight == min(min(matchWeight, mismatchWeight), gapWeight) {
		return matchWeight as f64 * length1 as f64
	}

	if length1 < length2 {
        let tmp = string2;
        string2 = string1;
        string1 = tmp;
        let tmp = length2;
        length2 = length1;
        length1 = tmp;
	};

	let mut D = vec![0.0; length1 + 1];
	let mut V_current = vec![0.0; length1 + 1];
	let mut V_previous = vec![0.0; length1 + 1];

	let mut distance = 0.0;

	for j in 1..(length1 + 1) {
		V_current[j] = gapWeight + spaceWeight*j as f64;
		D[j] = std::i32::MAX as f64;
	};

	for i in 1..(length2 + 1) {
		let char2 = string2.get(i-1..i).unwrap();

        let V_previous = V_current.clone(); // TODO there might be issues here

		V_current[0] = (gapWeight + (spaceWeight*i as f64)) as f64;
		let mut I = std::i32::MAX as f64;

		for j in 1..(length1 + 1) {
			let char1 = string1.get(j-1..j).unwrap();

			if j <= length2 {
				I = min(I, V_current[j-1]+gapWeight) + spaceWeight;
			} else {
				I = min(I, V_current[j-1]+gapWeight *abbreviation_scale) + spaceWeight * abbreviation_scale;
			};
			D[j] = min(D[j], V_previous[j]+gapWeight) + spaceWeight;

			let mut M: f64;
			if char2 == char1 {
				M = V_previous[j-1] + matchWeight;
			} else {
				M = V_previous[j-1] + mismatchWeight;
			};

			V_current[j] = min(min(I, D[j]), M)
		};
	};
	distance = V_current[length1];

	distance
}
fn normalised_affine_gap_distance(string1: String, string2: String) -> f64 {

	let length1 = string1.len();
	let length2 = string2.len();

	let normalizer = length1 + length2;

	let distance = affine_gap_distance(string1, string2);

	distance / normalizer as f64
}
