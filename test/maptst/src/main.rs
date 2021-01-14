use std::collections::HashMap;
use std::collections::BTreeMap;
use std::collections::HashSet;
use std::collections::BTreeSet;


fn main() {
	let mut hmap = HashMap::new();
	let mut bmap = BTreeMap::new();
	let mut hset = HashSet::new();
	let mut bset = BTreeSet::new();

	hmap.insert(3,"c");
	hmap.insert(1,"a");
	hmap.insert(2,"b");
	hmap.insert(4,"d");
	hmap.insert(5,"e");

	bmap.insert(3,"c");
	bmap.insert(1,"a");
	bmap.insert(2,"b");
	bmap.insert(4,"d");
	bmap.insert(5,"e");

	println!("{:?}", hmap);
	println!("{:?}", bmap);

	hset.insert("ccsss");
	hset.insert("xx22");
	hset.insert("vbvss");
	hset.insert("ccsss");

	bset.insert("ccsss");
	bset.insert("xx22");
	bset.insert("vbvss");
	bset.insert("ccsss");

	println!("{:?}", hset);
	println!("{:?}", bset);

	return;
}