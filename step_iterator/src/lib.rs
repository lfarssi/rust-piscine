use std::ops::Add;
pub struct StepIterator<T> {
	beg:T,
	end:T,
	step:T,
	finished:bool,
    
}

impl<T> StepIterator<T> where T: Add<Output=T> + Copy + PartialOrd {
	pub fn new(beg: T, end: T, step: T) -> Self {
		Self {
			beg,
			end,
			step,
			finished: false,
		}
	}
}

impl<T> std::iter::Iterator for StepIterator<T> where T: Add<Output=T> + Copy + PartialOrd {
	type Item= T;

	fn next(&mut self) -> Option<Self::Item> {
		if self.finished {
			return None;
		}
		let curr = self.beg;
		if curr > self.end {
			self.finished= true;
			return None;
		}
		let next  = self.beg + self.step ;
		if next > self.end{
			self.finished= true;
		} else {
			self.beg = next;
		}
		Some(self.beg)
	}
}