extern crate qmlrs;

pub struct Factorial;
impl Factorial {
    pub fn calculate(&self, x: i64) -> i64 {
        (1..x+1).fold(1, |t,c| t * c)
    }
}

Q_OBJECT! { Factorial:
    slot fn calculate(i64);
    // signal fn test();
}
