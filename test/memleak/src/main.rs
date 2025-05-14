use std::alloc::{GlobalAlloc, Layout};
use std::cell::UnsafeCell;
use std::ptr::null_mut;
use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};
use std::mem::ManuallyDrop;

const ARENA_SIZE: usize = 128 * 1024;
const MAX_SUPPORTED_ALIGN: usize = 4096;
#[repr(C, align(4096))] // 4096 == MAX_SUPPORTED_ALIGN
struct SimpleAllocator {
    arena: UnsafeCell<[u8; ARENA_SIZE]>,
    remaining: AtomicUsize, // we allocate from the top, counting down
}

#[global_allocator]
static ALLOCATOR: SimpleAllocator = SimpleAllocator {
    arena: UnsafeCell::new([0x55; ARENA_SIZE]),
    remaining: AtomicUsize::new(ARENA_SIZE),
};

unsafe impl Sync for SimpleAllocator {}

#[allow(unsafe_op_in_unsafe_fn)]
unsafe fn _write_str(s :&str) {
	let _ptr :*const u8 = s.as_bytes().as_ptr();
	libc::write(2,_ptr as *const libc::c_void,s.len() as u32);
}

#[allow(unused_mut)]
#[allow(unsafe_op_in_unsafe_fn)]
unsafe fn _write_val(val :u64, ishex :bool) {
	let mut cbuf :[u8;32] = [0;32];
	let mut clen :usize = 0;
	let mut obuf :[u8;32] = [0;32];
	let mut cval :u64 = val;

	if ishex {
		while cval > 0 {
			let curval :u8 = (cval & 0xf) as u8;
			if curval >= 0 && curval <= 9 {
				cbuf[clen] = b'0' + curval;
			} else {
				cbuf[clen] = b'a' + (curval - 10);
			}
			clen += 1;
			cval >>= 4;
		}

		cbuf[clen] = b'x';
		clen += 1;
		cbuf[clen] = b'0';
		clen += 1
	} else {
		while cval > 0 {
			let curval :u8 = (cval % 10) as u8;
			cbuf[clen] = b'0' + curval;
			clen += 1;
			cval = cval / 10;
		}
	}

	if clen == 0 {
		obuf[0] = b'0';
		clen += 1;
	} else {
		for i in 0..clen {
			obuf[i] = cbuf[clen - i-1];
		}
	}
	let _ptr :*const u8 = obuf.as_ptr();
	libc::write(2,_ptr as *const libc::c_void,clen as u32);
	return;
}

unsafe impl GlobalAlloc for SimpleAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = layout.size();
        let align = layout.align();

        // `Layout` contract forbids making a `Layout` with align=0, or align not power of 2.
        // So we can safely use a mask to ensure alignment without worrying about UB.
        let align_mask_to_round_down = !(align - 1);

        if align > MAX_SUPPORTED_ALIGN {
            return null_mut();
        }

        let mut allocated = 0;
        if self
            .remaining
            .fetch_update(Relaxed, Relaxed, |mut remaining| {
                if size > remaining {
                    return None;
                }
                remaining -= size;
                remaining &= align_mask_to_round_down;
                allocated = remaining;
                Some(remaining)
            })
            .is_err()
        {
            return null_mut();
        };
        let ptr :*mut u8 = self.arena.get().cast::<u8>().add(allocated);
        _write_str("allocate size[");
        _write_val(size as u64,true);
        _write_str(":");
        _write_val(size as u64,false);
        _write_str("] retptr [");
        _write_val(ptr as u64, true);
        _write_str("]\n");
        ptr
    }
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
    	_write_str("dealloc retptr[");
    	_write_val(_ptr as u64, true);
    	_write_str("]\n");
    }
}

fn main() {
    let _s = format!("allocating a string!");
    let _n = ManuallyDrop::new(format!("cc str"));
    let currently = ALLOCATOR.remaining.load(Relaxed);
    println!("_s {} {:p}", _s,_s.as_bytes().as_ptr());
    println!("_n {:?} {:p}", _n,(&_n).as_bytes().as_ptr());
    drop(_s);
    println!("allocated so far: {}", ARENA_SIZE - currently);
}