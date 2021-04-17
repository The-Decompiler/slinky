use std::collections::HashMap;

fn compress(data: &[u8]) -> Vec<u32> {
	let mut table: HashMap<Vec<u8>, u32> = (0u32..=255)
		.map(|i| (vec![i as u8], i))
		.collect();

	let mut p = Vec::new();
	let mut compressed = Vec::new();

	for &c in data {
		let mut pc = p.clone();
		pc.push(c);

		if table.contains_key(&pc) {
			p = pc;
		} else {
			compressed.push(table[&p]);

			table.insert(pc, table.len() as u32);
			p.clear();
			p.push(c);
		}
	}

	if !p.is_empty() {
		compressed.push(table[&p]);
	}

	compressed
}

fn decompress(data: &[u32]) -> Vec<u8> {
	let mut table: HashMap<u32, Vec<u8>> = (0u32..=255)
		.map(|i| (i, vec![i as u8]))
		.collect();

	let mut old = table[&data[0]].clone();
	let mut decompressed = old.clone();

	for &new in &data[1..] {
		let s = if !table.contains_key(&new) {
			let mut s = old.clone();
			s.push(s[0]);
			s
		} else {
			table[&new].clone()
		};

		decompressed.extend(&s);

		old.push(s[0]);
		table.insert(table.len() as u32, old);

		old = s;
	}

	decompressed
}

fn main() {
	let compressed = compress("BABAABAAA".as_bytes());
	println!("{:?}", compressed);

	let decompressed = decompress(&compressed);
	let decompressed = String::from_utf8(decompressed).unwrap();
	println!("{}", decompressed);
}
