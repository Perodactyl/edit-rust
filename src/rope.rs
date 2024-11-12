use std::mem;

#[derive(Debug, Clone)]
struct RopeTrunkNode {
	child0: Box<RopeNode>,
	child1: Box<RopeNode>,
	length: usize,
} impl RopeTrunkNode {
	///Converts a Rope subtree into a single Leaf node.
	pub fn collapse(self) -> RopeLeafNode {
		RopeLeafNode(self.collapse_bytes())
	}

	///Converts a Rope subtree into a flattened version
	pub fn collapse_bytes(self) -> Vec<u8> {
		let mut out = self.child0.collapse();
		out.append(&mut self.child1.collapse());

		out
	}
}

#[derive(Debug, Clone)]
struct RopeLeafNode(Vec<u8>);
impl RopeLeafNode {
	///Splits a leaf node into a trunk node at the given index. `i` is the index of the *first character* in the *second node*.
	pub fn split(self, i: usize) -> RopeTrunkNode {
		let original_length = self.0.len();
		let (left, right) = self.0.split_at(i);

		RopeTrunkNode {
			child0: Box::new(RopeNode::Leaf(RopeLeafNode(left.to_owned()))),
			child1: Box::new(RopeNode::Leaf(RopeLeafNode(right.to_owned()))),
			length: original_length,
		}
	}
	///Appends another leaf node to this one.
	pub fn append(&mut self, mut other: RopeLeafNode) {
		self.0.append(&mut other.0);
	}
}

impl Into<RopeLeafNode> for RopeTrunkNode {
	fn into(self) -> RopeLeafNode {
		self.collapse()
	}
}

impl Into<Vec<u8>> for RopeTrunkNode {
	fn into(self) -> Vec<u8> {
		self.collapse_bytes()
	}
}

#[derive(Debug, Clone)]
enum RopeNode {
	None,
	Trunk(RopeTrunkNode),
	Leaf(RopeLeafNode),
} impl RopeNode {
	pub fn len(&self) -> usize {
		match self {
			RopeNode::Leaf(s) => s.0.len(),
			RopeNode::Trunk(t) => t.length,
			RopeNode::None => 0,
		}
	}
	pub fn byte_at(&self, i: usize) -> u8 {
		match self {
			RopeNode::Leaf(l) => l.0[i],
			RopeNode::Trunk(t) => if i < t.child0.len() {
					t.child0.byte_at(i)
				} else {
					t.child1.byte_at(i - t.child0.len())
				},
			RopeNode::None => 0,
		}
	}
	
	///Replaces `self` with a RopeNode::None and returns the original value.
	fn take(&mut self) -> RopeNode {
		mem::replace(self, RopeNode::None)
	}

	pub fn insert_byte(&mut self, byte: u8, i: usize) {
		match self {
			RopeNode::Leaf(l) => {
				if i == l.0.len() {
					l.0.push(byte);
				} else {
					//Temporarily replaces self with a RopeNode::None so that we can mutate what enum branch `self` falls under.
					let inner = self.take();
					if let RopeNode::Leaf(l) = inner {
						let mut new_trunk ;

						if i != 0 {
							new_trunk = l.split(i);
						} else {
							new_trunk = RopeTrunkNode {
								length: l.0.len(),
								child0: Box::new(RopeNode::None),
								child1: Box::new(RopeNode::Leaf(l)),
							}
						}

						new_trunk.child0.insert_byte(byte, i);
						*self = RopeNode::Trunk(new_trunk);
					} else {
						unreachable!()
					}
				}
			},
			RopeNode::Trunk(t) => {
				t.length += 1;
				if i < t.child0.len() {
					t.child0.insert_byte(byte, i);
				} else {
					t.child1.insert_byte(byte, i - t.child0.len());
				}
			},
			RopeNode::None => {
				if i == 0 {
					*self = RopeNode::Leaf(RopeLeafNode(vec![byte]));
				} else {
					panic!("Cannot insert a byte past the end of RopeNode::None");
				}
			}
		}
	}

	pub fn insert_bytes(&mut self, bytes: &[u8], i: usize) {
		match self {
			RopeNode::Leaf(l) => {
				if i == l.0.len() {
					l.0.extend(bytes);
				} else {
					//Temporarily replaces self with a RopeNode::None so that we can mutate whatl enum branch `self` falls under.
					let inner = self.take();
					if let RopeNode::Leaf(l) = inner {
						let mut new_trunk ;

						if i != 0 {
							new_trunk = l.split(i);
						} else {
							new_trunk = RopeTrunkNode {
								length: l.0.len(),
								child0: Box::new(RopeNode::None),
								child1: Box::new(RopeNode::Leaf(l)),
							}
						}

						new_trunk.child0.insert_bytes(bytes, i);
						*self = RopeNode::Trunk(new_trunk);
					} else {
						unreachable!()
					}
				}
			},
			RopeNode::Trunk(t) => {
				t.length += bytes.len();
				if i < t.child0.len() {
					t.child0.insert_bytes(bytes, i);
				} else {
					t.child1.insert_bytes(bytes, i - t.child0.len());
				}
			}
			RopeNode::None => {
				if i == 0 {
					*self = RopeNode::Leaf(RopeLeafNode(Vec::from(bytes)));
				} else {
					panic!("Cannot insert a byte past the end of RopeNode::None");
				}
			},
		}
	}

	pub fn collapse(self) -> Vec<u8> {
		match self {
			RopeNode::Leaf(l) => l.0,
			RopeNode::Trunk(t) => t.collapse_bytes(),
			RopeNode::None => vec![],
		}
	}
}

impl Into<RopeLeafNode> for RopeNode {
	fn into(self) -> RopeLeafNode {
		match self {
			RopeNode::Leaf(l) => l,
			RopeNode::Trunk(RopeTrunkNode {child0, child1, ..}) => {
				let mut out: RopeLeafNode  = (*child0).into();

				out.append((*child1).into());

				out
			},
			RopeNode::None => RopeLeafNode(vec![])
		}
	}
}

impl From<String> for RopeNode {
	fn from(value: String) -> Self {
		RopeNode::Leaf(RopeLeafNode(value.into_bytes()))
	}
}

impl From<Vec<u8>> for RopeNode {
	fn from(value: Vec<u8>) -> Self {
		RopeNode::Leaf(RopeLeafNode(value))
	}
}

#[derive(Debug, Clone)]
pub struct Rope {
	head: Box<RopeNode>
}
impl Rope {
	pub fn new() -> Self {
		Rope {
			head: Box::new(RopeNode::None)
		}
	}
	pub fn byte_at(&self, i: usize) -> u8 {
		self.head.byte_at(i)
	}
	
	pub fn insert_byte(&mut self, byte: u8, i: usize) {
		self.head.insert_byte(byte, i);
	}
	
	pub fn insert_bytes(&mut self, bytes: &[u8], i: usize) {
		self.head.insert_bytes(bytes, i);
	}

	pub fn len(&self) -> usize {
		self.head.len()
	}

	///Destroys self, returning the flattened contents of this tree.
	pub fn collapse(self) -> Vec<u8> {
		self.head.collapse()
	}
}

pub struct RopeIterator<'a> {
	rope: &'a Rope,
	index: usize,
} impl<'a> Iterator for RopeIterator<'a> {
	type Item = u8;
	fn next(&mut self) -> Option<Self::Item> {
		if self.index > self.rope.len() {
			None
		} else {
			let v = self.rope.byte_at(self.index);
			self.index += 1;

			Some(v)
		}
	}
}

impl<'a> IntoIterator for &'a Rope {
	type IntoIter = RopeIterator<'a>;
	type Item = u8;

	fn into_iter(self) -> Self::IntoIter {
		RopeIterator {
			rope: self,
			index: 0,
		}
	}
}

#[cfg(test)]
mod test {
    use super::Rope;

	#[test]
	fn insert_byte() {
		let mut r = Rope::new();
		r.insert_byte(b'c', 0);
		r.insert_byte(b'b', 0);
		r.insert_byte(b'a', 0);
		println!("{:?}", r);
		assert_eq!(r.collapse(), b"abc");
	}
	
	#[test]
	fn insert_bytes() {
		let mut r = Rope::new();

		r.insert_bytes(b"Hello, World!", 0);
		println!("{:?}", r);
		assert_eq!(r.clone().collapse(), b"Hello, World!");

		r.insert_bytes(b" Rusty", 6);
		println!("{:?}", r);
		assert_eq!(r.collapse(), b"Hello, Rusty World!");
	}

	#[test]
	#[should_panic]
	fn insert_past_end() {
		let mut r = Rope::new();

		r.insert_byte(b'A', 1);
	}

	
}