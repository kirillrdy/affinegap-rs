use std::cmd::min;

fn main() {
    println!("Hello, world!");
}

fn AffineGapDistance(string_a: String, string_b: String) -> f64 {

	let matchWeight = 1;
	let mismatchWeight = 11;
	let gapWeight = 10;
	let spaceWeight = 7;
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

	let D: [f64, length1 + 1] = [0; length1 + 1];
	V_current := make([]float64, length1+1)
	V_previous := make([]float64, length1+1)

	var distance float64

	for j := 1; j < (length1 + 1); j++ {
		V_current[j] = gapWeight + float64(spaceWeight*j)
		D[j] = math.MaxInt32 //TODO maybe not 32
	}

	for i := 1; i < (length2 + 1); i++ {
		char2 := string2[i-1]

		//for _ in range(0, length1 + 1) :
		copy(V_previous, V_current)

		V_current[0] = gapWeight + float64(spaceWeight*i)
		I := float64(math.MaxInt32)

		for j := 1; j < (length1 + 1); j++ {
			char1 := string1[j-1]

			if j <= length2 {
				I = math.Min(I, V_current[j-1]+gapWeight) + spaceWeight
			} else {
				I = (math.Min(I, V_current[j-1]+gapWeight*abbreviation_scale) + spaceWeight*abbreviation_scale)
			}
			D[j] = math.Min(D[j], V_previous[j]+gapWeight) + spaceWeight

			var M float64
			if char2 == char1 {
				M = V_previous[j-1] + matchWeight
			} else {
				M = V_previous[j-1] + mismatchWeight
			}

			V_current[j] = math.Min(math.Min(I, D[j]), M)
		}
	}
	distance = V_current[length1];

	distance
}
fn NormalisedAffineGapDistance(string1: String, string2: String) -> f64 {

	let length1 = string1.len();
	let length2 = string2.len();

	let normalizer = length1 + length2;

	let distance = AffineGapDistance(string1, string2);

	distance / normalizer as f64
}
