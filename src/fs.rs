// use alloc::string::{String, ToString};
// use alloc::vec::Vec;

// use lazy_static::lazy_static;
// use spin::Mutex;

// lazy_static! {
// 	static ref FS_ROOT: Mutex<Directory> = Mutex::new(Directory {
// 		name:     "/".to_string(),
// 		contents: Vec::new(),
// 	});
// }

// pub enum NodeType<'a> {
// 	Directory(&'a Directory),
// 	File(&'a File),
// }

// impl<'a> NodeType<'a> {
// 	fn is_directory(&self) -> bool {
// 		match self {
// 			NodeType::Directory(_) => true,
// 			NodeType::File(_) => false,
// 		}
// 	}
// }

// pub struct File {
// 	name:     String,
// 	contents: String,
// }

// pub struct Directory<'a> {
// 	name:     String,
// 	contents: Vec<NodeType<'a>>,
// }

// pub fn open<'a>(path: String) -> Option<&'a NodeType<'a>> {
// 	let path = path
// 		.split('/')
// 		.map(ToString::to_string)
// 		.collect::<Vec<String>>();

// 	let fs = FS_ROOT.lock();

// 	let mut node: NodeType = NodeType::Directory(&fs);

// 	for (index, path_component) in path.iter().enumerate() {
// 		if node.is_directory() && index != path.len() - 1 {
//             for item
// 		} else {
// 			return None;
// 		}
// 	}

// 	todo!()
// }
