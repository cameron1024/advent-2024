#[derive(Debug, Clone)]
pub enum UpdateData<'a> {
    Unmodified(&'a str),
    Fixed(Vec<&'a str>),
}

impl<'a> UpdateData<'a> {
    pub fn iter(&'a self) -> impl Iterator<Item = &'a str> {
        self.into_iter()
    }
}

impl<'a> IntoIterator for &'a UpdateData<'a> {
    type Item = &'a str;
    type IntoIter = UpdateDataIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            UpdateData::Unmodified(str) => UpdateDataIter::Unmodified(str.split(",")),
            UpdateData::Fixed(vec) => UpdateDataIter::Fixed(vec.iter().copied()),
        }
    }
}

pub enum UpdateDataIter<'a> {
    Unmodified(std::str::Split<'a, &'static str>),
    Fixed(std::iter::Copied<std::slice::Iter<'a, &'a str>>),
}

impl<'a> Iterator for UpdateDataIter<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Unmodified(iter) => iter.next(),
            Self::Fixed(iter) => iter.next(),
        }
    }
}
