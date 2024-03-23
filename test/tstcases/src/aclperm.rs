/*use for the acl perm*/
use winapi::um::aclapi::{GetNamedSecurityInfoA};
use winapi::um::winnt::{PACL};
use winapi::um::accctrl::{AccFree};

pub struct FilePerm {
	fname String,
	dacl :*mut PACL,
}

impl Drop for FilePerm {
	fn drop(&mut self) {
		if self.dacl != std::ptr::null_mut::<PACL>() {
			AccFree(self.dacl);
		}
		self.dacl = std::ptr::null_mut::<PACL>();
		return;
	}
}

impl FilePerm {
	pub fn new(n :&str) -> Result<Self> {
		let mut retv :Self = Self {
			fname : n.to_string(),
			dacl : std::ptr::null_mut<PACL>(),
		};
		
	}
}