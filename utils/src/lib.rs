use std::{
    fmt::Debug,
    fs::File,
    io::{BufRead, BufReader, Lines},
    str::FromStr,
};

fn open_file(file_path: &str) -> File {
    File::open(file_path).unwrap()
}

pub struct AocBufReader {
    iter: Lines<BufReader<File>>,
}

impl AocBufReader {
    fn from_file(file_handle: File) -> AocBufReader {
        AocBufReader {
            iter: BufReader::new(file_handle).lines(),
        }
    }

    pub fn from_string(file_path: &str) -> AocBufReader {
        AocBufReader::from_file(open_file(file_path))
    }
}

impl Iterator for AocBufReader {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(result) => match result {
                Ok(line) => Some(line),
                Err(error) => panic!("{}", error),
            },
            None => None,
        }
    }
}

pub fn parse_iter<T: FromStr + Debug>(
    input: impl Iterator<Item = String>,
) -> impl Iterator<Item = T>
where
    <T as FromStr>::Err: Debug,
{
    input.map(|x| x.parse::<T>().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ints() {
        let buffer = AocBufReader::from_string("src/data/test_parse_ints.txt");
        assert_eq!(
            parse_iter::<usize>(buffer).collect::<Vec<_>>(),
            vec![1, 2, 3, 4]
        );
    }
}
