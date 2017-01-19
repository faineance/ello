

pub struct Fresh(usize);


impl Iterator for Fresh {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        return Some(self.0 + 1);
    }
}
