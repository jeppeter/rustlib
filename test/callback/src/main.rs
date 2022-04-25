type Callback = fn();

struct Processor {
    callback: Callback,
}

impl Processor {
    fn set_callback(&mut self, c: Callback) {
        self.callback = c;
    }

    fn process_events(&self) {
        (self.callback)();
    }
    pub fn add_f(&self,idx :i32, v :&mut Vec<String>) {
        if idx > 1 {
            self.add_f(idx - 1, v);
        }
        v.push(format!("{}",idx));
        return;
    }
}

fn simple_callback() {
    println!("hello world!");
}

fn main() {
    let mut p = Processor {
        callback: simple_callback,
    };
    let mut v :Vec<String> = Vec::new();
    p.set_callback(simple_callback);
    p.process_events(); // hello world!
    p.add_f(3,&mut v);
    println!("{:?}", v);
}