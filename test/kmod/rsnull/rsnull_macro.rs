
macro_rules! _inner_debug {
	($level:expr,$lvlstr:expr,$($args:tt)+) => {
		match format_args!("{}:{}",file!(),line!()) {
			_c => {
				match format_args!($($args)+) {
					_s => {
						match format_args!("{} [{}] {}",_c,$lvlstr,_s) {
							_ws => {
								pr_debug!("{}",_ws);
							}
						}
					}
				}
			}
		}		
	};
}

macro_rules! _insert_nb {
	($nb :expr,$c :expr) => {
		let _ = $nb.try_push(' ' as u8);
		let _ = $nb.try_push('0' as u8);
		let _ = $nb.try_push('x' as u8);
		let mut _sidx : usize = 0;
		while _sidx < 2 {
			let _cv = (($c >> (4*(1 - _sidx))) & 0xf ) as u8;
			if  _cv <= 9 {
				let _ = $nb.try_push((('0' as u8) + _cv) as u8);
			} else if _cv >= 10 && _cv <= 15 {
				let _ = $nb.try_push((('a' as u8) + _cv - 10) as u8);
			}
			_sidx += 1;
		}
	}
}

macro_rules! _insert_prefix {
	($nb:expr,$idx :expr) => {
		let _ = $nb.try_push('0' as u8);
		let _ = $nb.try_push('x' as u8);
		let mut _sidx :usize = 0;
		while _sidx < 8 {
			let _cv = (($idx >> (4*(7 - _sidx))) & 0xf ) as u8;
			if  _cv <= 9 {
				let _ = $nb.try_push((('0' as u8) + _cv) as u8);
			} else if _cv >= 10 && _cv <= 15 {
				let _ = $nb.try_push((('a' as u8) + _cv - 10) as u8);
			}
			_sidx += 1;
		}
		let _ = $nb.try_push(':' as u8);
		let _ = $nb.try_push(' ' as u8);
	}
}

macro_rules! _inner_buffer_debug {
	($level:expr,$lvlstr:expr,$ptr:expr,$len:expr,$($args:tt)+) => {
		let _len :usize = $len as usize;
		let _ptr :*const u8 = $ptr as *const u8;
		match format_args!("{}:{}",file!(),line!()) {
			_c => {
				match format_args!("ptr {:p} size {}:0x{:x}",_ptr,_len,_len){
					_ls => {
						match format_args!($($args)+) {
							_s => {
								match format_args!("{} [{}] {} {}",_c,$lvlstr,_ls,_s) {
									_ws => {
										pr_debug!("{}",_ws);
									}
								}
							}
						}					
					}
				}
			}
		}
		let mut _bs :Vec<u8> = Vec::new();
		let mut _idx :usize = 0;
		let mut _lasti :usize = 0;
		let mut _nb :u8;

		while _idx < _len {
			if (_idx % 16) == 0 {
				if _idx > 0 {
					match format_args!("    ") {
						_cc => {
							if let Some(_ss) = _cc.as_str() {
								for _nc in _ss.as_bytes().iter() {
									let _ = _bs.try_push(*_nc);
								}
							}
						}
					}
					while _lasti < _idx {
						unsafe {
							_nb = *_ptr.offset(_lasti as isize);
						}
						if _nb >= 0x20 && _nb <= 0x7e {
							let _ = _bs.try_push(_nb as u8);
						} else {
							let _ = _bs.try_push('.' as u8);
						}
						_lasti += 1;
					}
					let _ = _bs.try_push(0);
					let _ros = CStr::from_bytes_with_nul(&_bs);
					if _ros.is_ok() {
						let _cs = _ros.unwrap();
						pr_debug!("{}",_cs);
					} else {
						debug_trace!("not valid for _bs len[{}] {:?}",_bs.len(),_ros.err().unwrap());
					}
					_bs = Vec::new();
				}        		
				_insert_prefix!(_bs,_idx);
			}
			unsafe {_nb = *_ptr.offset(_idx as isize);}
			_insert_nb!(_bs,_nb);
			_idx += 1;
		}

		if _lasti != _idx {
			while (_idx % 16) != 0 {
				match format_args!("     ") {
					_cc => {
						if let Some(_ss) = _cc.as_str() {
							for _nc in _ss.as_bytes().iter() {
								let _ = _bs.try_push(*_nc);
							}

						}
					}
				}
				_idx += 1;
			}
			match format_args!("    ") {
				_cc => {
					if let Some(_ss) = _cc.as_str() {
						for _nc in _ss.as_bytes().iter() {
							let _ = _bs.try_push(*_nc);
						}
					}
				}
			}

			while _lasti < _len {
				unsafe {_nb = *_ptr.offset(_lasti as isize);}
				if _nb >= 0x20 && _nb <= 0x7e {
					let _ = _bs.try_push(_nb as u8);
				} else {
					let _ = _bs.try_push('.' as u8);
				}
				_lasti += 1;
			}
		} 

		if _bs.len() > 0 {
			let _ = _bs.try_push(0);
			let _ros = CStr::from_bytes_with_nul(&_bs);
			if _ros.is_ok() {
				let _cs = _ros.unwrap();
				pr_debug!("{}",_cs);
			} else {
				pr_debug!("not valid for _bs len{}",_bs.len());
			}
			_bs= Vec::new();
		}
	}
}

macro_rules! debug_trace {
	($($args:tt)+) => {
		_inner_debug!(50,"TRACE",$($args)+);
	};
}

macro_rules! error_trace {
	($($args:tt)+) => {
		_inner_debug!(10,"ERROR",$($args)+);
	};
}

macro_rules! debug_buffer_trace {
	($ptr:expr,$len:expr,$($args:tt)+) => {
		_inner_buffer_debug!(50,"TRACE",$ptr,$len,$($args)+);
	};
}