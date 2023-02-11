/// An iterator over indices in a cluster
pub enum Indices<'a> {
    SliceCopied(std::iter::Copied<std::slice::Iter<'a, usize>>),
    Chain(Box<std::iter::Chain<Indices<'a>, Indices<'a>>>),
}

impl Iterator for Indices<'_> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::SliceCopied(iter) => iter.next(),
            Self::Chain(iter) => iter.next(),
        }
    }
}
