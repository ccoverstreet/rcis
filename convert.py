# This script converts the periodic table csv to a rust phf_map

def main():
	with open("data/periodic_table.csv") as f:
		field_map, all_data = parse_csv(f)

	out = create_rust_phf_map(field_map, all_data)

	with open("src/pt.rs", "w") as f:
		f.write(out)

def parse_csv(f):
	field_map = {}

	all_data = []

	for i, line in enumerate(f):
		if i == 0:
			fields = line.strip().split(",")
			for j, f in enumerate(fields):
				field_map[f] = j

			continue

		data = line.strip().split(",")
		for i, d in enumerate(data):
			if d.isnumeric():
				if "." in d:
					data[i] = float(d)
				else:
					data[i] = int(d)


		all_data.append(data)

	return field_map, all_data

def create_rust_phf_map(fields, all_data):
	pre = """use phf::phf_map;
use std::fmt;

pub struct Element {
	pub name: &'static str,
	pub symbol: &'static str,
	pub z: u32,
	pub mass: f64, // amu
	pub radius: Option<f64> // Angstrom
}

impl fmt::Display for Element {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{} ({})\nZ = {}\nMass = {}\nRadius = {:?} Angstrom\n", self.name, self.symbol, self.z, self.mass, self.radius)
	}
}



	"""

	out = pre + "pub const ELEMENTS: phf::Map<&'static str, Element> = phf_map! {\n"

	for d in all_data:
		content = "name: \"{}\", symbol: \"{}\", z: {}, mass: {}, radius: {}".format(
			d[fields["Element"]],
			d[fields["Symbol"]],
			d[fields["NumberofProtons"]],
			float(d[fields["AtomicMass"]]),
			"Some({})".format(str(float(d[fields["AtomicRadius"]]))) if d[fields["AtomicRadius"]] != "" else "None"
		)

		out += "\t\"" + d[fields["Symbol"]]\
			+ "\" => Element{ " + content + " },\n"

	out += "\n};"

	return out


if __name__ == "__main__":
	main()


