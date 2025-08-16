#[derive(Debug)]
pub struct Numbers<'a> {
    numbers: &'a [u32],
}

impl<'a> Numbers<'a> {
    pub fn new(numbers: &'a [u32]) -> Self {
        Self {
            numbers
        }
    }

    pub fn list(&self) -> &[u32] {
        self.numbers
    }

    pub fn latest(&self) -> Option<u32> {
        self.numbers[self.numbers.len()-1].copied()
    }

    pub fn highest(&self) -> Option<u32> {
        let res = self.numbers.iter().max()?;
        Some(*res)
    }

    pub fn highest_three(&self) -> Vec<u32> {
        let mut res = Vec::new();
        let mut nums = self.numbers.to_vec();
        nums.sort();
        // println!("__________________{:?}", nums);
        for num in nums.iter().rev().take(3){
            res.push(*num);
        }
        res
    }
}