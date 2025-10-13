pub fn num_arr<N>(num: N) -> String
where N: Shl + Mul
{
    let zeroes = num.leading_zeros() as u64;
    let sub = 64 - zeroes;
    let units: usize = BitsCycle::new(sub).units();
    let mut split = SplitNum::new(num, units);

    unsafe  {String::from_utf8_unchecked(split.split()) }
}

enum UnitCycle<N> {
    First1(N),
    First2(N),
    First3(N),
    First4(N),
    Second1(N),
    Second2(N),
    Second3(N),
    Third1(N),
    Third2(N),
    Third3(N),
}

impl UnitCycle {
    const fn pass(&mut self) {
        use UnitCycle::*;

        match self {
            First1(num) => *self = First2(*num),
            First2(num) => *self = First3(*num),
            First3(num) => *self = First4(*num),
            First4(num) => *self = Second1(*num + 1),
            Second1(num) => *self = Second2(*num),
            Second2(num) => *self = Second3(*num),
            Second3(num) => *self = Third1(*num + 1),
            Third1(num) => *self = Third2(*num),
            Third2(num) => *self = Third3(*num),
            Third3(num) => *self = First1(*num + 1),
        }
    }

    const fn units(&self) -> usize {
        use UnitCycle::*;

        match self {
            First1(num) | First2(num) | First3(num) | First4(num) | Second1(num) | Second2(num)
            | Second3(num) | Third1(num) | Third2(num) | Third3(num) => *num as usize,
        }
    }
}

struct BitsCycle<N> {
    cycle: UnitCycle,
    delim: N,
}

impl BitsCycle<N> {
    const fn new(delim: N) -> Self {
        Self {
            cycle: UnitCycle::First1(1),
            delim,
        }
    }

    const fn units(mut self) -> usize {
        while self.delim > 0 {
            self.cycle.pass();
            self.delim -= 1;
        }

        self.cycle.units()
    }
}

struct SplitNum<N> {
    units: usize,
    num: N,
}

const ASCII_IDX: [u8; 10] = [b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9'];
impl SplitNum<N> {
    fn new(num: N, units: usize) -> Self {
        Self { num, units }
    }

    fn split(&mut self) -> [u8; 10] {
        let mut units = self.units as u64;
        let mut arr = [10u8; 10];
        let mut idx = self.units - units as usize;
        while units > 0 {
            let residual = self.num % 10u64.pow(units as u32);
            units -= 1;
            let split_on = 10u64.pow(units as u32);

            let char = (residual / split_on) as u8;

            arr[idx] = ASCII_IDX[char as usize];
            idx += 1;
        }

        arr
    }
}
