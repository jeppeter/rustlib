#![feature(macro_rules)]

macro_rules! recurrence {
    ( a[n]: $sty:ty = $($inits:expr),+ , ... , $recur:expr ) => {
        {
            struct Recurrence {
                mem: [u64, ..2],
                pos: uint,
            }

            struct IndexOffset<'a> {
                slice: &'a [u64, ..2],
                offset: uint,
            }

            impl<'a> Index<uint, u64> for IndexOffset<'a> {
                #[inline(always)]
                fn index<'b>(&'b self, index: &uint) -> &'b u64 {
                    let real_index = *index - self.offset + 2;
                    &self.slice[real_index]
                }
            }

            impl Iterator<u64> for Recurrence {
                #[inline]
                fn next(&mut self) -> Option<u64> {
                    if self.pos < 2 {
                        let next_val = self.mem[self.pos];
                        self.pos += 1;
                        Some(next_val)
                    } else {
                        let next_val = {
                            let n = self.pos;
                            let a = IndexOffset { slice: &self.mem, offset: n };
                            (a[n-1] + a[n-2])
                        };

                        {
                            use std::mem::swap;

                            let mut swap_tmp = next_val;
                            for i in range(0, 2).rev() {
                                swap(&mut swap_tmp, &mut self.mem[i]);
                            }
                        }

                        self.pos += 1;
                        Some(next_val)
                    }
                }
            }

            Recurrence { mem: [0, 1], pos: 0 }
        }
    };
}

fn main() {
    let fib = recurrence!([a[n]: u64 = 0, 1, ..., a[n-1] + a[n-2]]);

    for e in fib.take(10) { println!("{}", e) }
}