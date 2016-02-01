extern crate qmlrs;
use math;

Q_OBJECT! { math::Factorial:
    slot fn calculate(i64);
    // signal fn test();
}
