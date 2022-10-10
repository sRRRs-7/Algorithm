
// schedule duplication count

use std::collections::BTreeMap;

pub fn main() {
    let times = [[0, 0], [10, 20], [50, 60], [10, 40], [5, 15], [5, 10], [25, 55]];

    let mut calender = MyCalendarThree::new();
    let mut result = Vec::new();

    for time in times {
        let res = calender.book(time[0], time[1]);
        result.push(res);
    };

    // println!("calender: {:?}", result);
}


#[derive(Default)]
struct MyCalendarThree {
    btree: BTreeMap<i32, i32>
}

impl MyCalendarThree {
    fn new() -> Self {
        Default::default()
    }

    fn book(&mut self, start: i32, end: i32) -> i32 {
        // hashmap entry or insert
        *self.btree.entry(start).or_insert(0) += 1;
        *self.btree.entry(end).or_insert(0) -= 1;

        let mut active = 0;
        let mut m = 0;
        for &t in self.btree.values() {
            // return maximum value in loop
            active += t;        // max state
            m = m.max(active);  // compare curr to active
        }

        m
    }
}