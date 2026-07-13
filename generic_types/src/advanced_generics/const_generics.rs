// Scenario:
// A type or function needs a number as part of its type.
//
// Thinking:
// Const generics allow values like array size to be part of a generic type.

#[derive(Debug)]
struct FixedBuffer<T, const N: usize> {
    values: [T; N],
}

impl<T: Copy, const N: usize> FixedBuffer<T, N> {
    fn new(default: T) -> FixedBuffer<T, N> {
        FixedBuffer {
            values: [default; N],
        }
    }

    fn len(&self) -> usize {
        N
    }
}

pub fn run() {
    println!("\n15. Const generics");

    let buffer = FixedBuffer::<i32, 4>::new(0);

    println!("Buffer: {:?}", buffer);
    println!("Buffer values: {:?}", buffer.values);
    println!("Buffer length: {}", buffer.len());
}
