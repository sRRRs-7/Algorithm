
use random_number::random;

pub fn main() {
    magic_circle();
}

#[derive(Debug)]
pub struct Square {
    square: Vec<Vec<i32>>
}

impl Square {
    fn new() -> Self {
        let mut sq = Vec::new();
        let mut state = Vec::new();
        while sq.len() != 9 {
            while state.len() != 9 {
                let rand: i32 = random!(1..=9);
                if state.contains(&rand) { continue; }
                state.push(rand);
            };
            sq.push(state.clone());
            state.clear();
        }
        Self { square: sq }
    }

    fn random_lack(&mut self) {

        for ii in 0..self.square.len() {
            let mut idx = Vec::new();
            for _ in 1..=9 {
                let rand: i32 = random!(0..=8);
                idx.push(rand);
            }
            idx.sort();
            idx.dedup();

            for i in &idx {
                self.square[ii][*i as usize] = 0;
            }
            idx.clear();
        };
    }

    fn complete(&mut self) {
        let mut candidates = Vec::new();
        let mut candidate = Vec::new();

        // candidate
        for x in &self.square {
            for n in 1..=9 {
                if !x.contains(&n) { candidate.push(n) }
            }
            candidates.push(candidate.clone());
            candidate.clear();
        }

        let mut is_complete = false;
        let state = &self.square;

        while !is_complete {
            // 0 substitute

            let t1 = self.total_row();
            let t2 = self.total_column();
            let t3 = self.total_block();
            if t1 && t2 && t3 { is_complete = true };
            break;

            // candidate pattern

        }

    }

    fn total_row(&mut self) -> bool {
        for row in &self.square {
            let total: i32 = row.iter().sum();
            if total != 45 { return false };
        };
        true
    }

    fn total_column(&mut self) -> bool {
        let mut total = 0;
        for ii in 0..9 {
            for i in 0..9 {
                total += self.square[i][ii]
            }
            if total != 45 { return false }
            total = 0;
        };
        true
    }

    fn total_block(&mut self) -> bool {
        let mut top = 0;
        let mut mid = 0;
        let mut bottom = 0;

        for ii in (3..9).step_by(3) {
            for i in 0..9 {
                top += self.square[ii-3][i];
                mid += self.square[ii-2][i];
                bottom += self.square[ii-1][i];

                if i == 2 || i == 5 || i == 8 {
                    let total = top + mid + bottom;
                    if total != 45 { return false };
                }

                top = 0;
                mid = 0;
                bottom = 0;
            }
        };
        true
    }

}



pub fn magic_circle() {
    let mut sq = Square::new();
    sq.random_lack();
    sq.complete();

    println!("{:?}", sq.square);
}