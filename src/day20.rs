use std::mem;
use std::str::FromStr;
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::ops::{Index, IndexMut};


#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Pixel {
    Light, Dark
}
impl Pixel {
    fn from_char(c: char) -> Option<Pixel> {
        match c {
            '#' => Some(Pixel::Light),
            '.' => Some(Pixel::Dark),
            _ => None,
        }
    }
}
impl Display for Pixel {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Pixel::Light => write!(f, "#"),
            Pixel::Dark => write!(f, "."),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Position(usize, usize);
impl Index<Position> for Image {
    type Output = Pixel;

    fn index(&self, index: Position) -> &Self::Output {
        &self.pixels[self.size.1 * index.0 + index.1]
    }
}
impl IndexMut<Position> for Image {
    fn index_mut(&mut self, index: Position) -> &mut Self::Output {
        &mut self.pixels[self.size.1 * index.0 + index.1]
    }
}

#[derive(Debug)]
enum ParseImageError {
    MissingAlgorithm, MissingImage, InvalidPixel(char), IncorrectAlgorithmSize, IncosistentRowLength, UnidentifiedTrailingData
}
impl Display for ParseImageError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ParseImageError::MissingAlgorithm => write!(f, "missing image enhancement algorithm rules"),
            ParseImageError::MissingImage => write!(f, "missing image"),
            ParseImageError::InvalidPixel(c) => write!(f, "invalid pixel '{}' was found", c),
            ParseImageError::IncorrectAlgorithmSize => write!(f, "the algorithms must specify exactly 512 bits"),
            ParseImageError::IncosistentRowLength => write!(f, "the length of the image rows are inconsistent"),
            ParseImageError::UnidentifiedTrailingData => write!(f, "unindentified trailing data"),
        }
    }
}
impl Error for ParseImageError {}

#[derive(Debug)]
struct Image {
    pixels: Vec<Pixel>,
    buffer: Vec<Pixel>,
    padding_pixel: Pixel,
    size: Position,
    algorithm: Vec<Pixel>,
}
impl FromStr for Image {
    type Err = ParseImageError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let block_delimiter = s.chars().nth(s.lines().next().unwrap().chars().count());
        let block_delimiter = match block_delimiter {
            Some('\r') => "\r\n\r\n",
            Some('\n') => "\n\n",
            _ => return Err(ParseImageError::MissingAlgorithm),
        };

        let mut blocks = s.split(block_delimiter);
        let algorithm = blocks.next().ok_or(ParseImageError::MissingAlgorithm)?;
        let pixels = blocks.next().ok_or(ParseImageError::MissingImage)?;
        if blocks.next().is_some() {
            return Err(ParseImageError::UnidentifiedTrailingData);
        }

        let algorithm = algorithm.trim().chars()
            .map(|c| {
                Pixel::from_char(c).ok_or(ParseImageError::InvalidPixel(c))
            })
            .collect::<Result<Vec<Pixel>, ParseImageError>>()?;
        if algorithm.len() != 512 {
            return Err(ParseImageError::IncorrectAlgorithmSize);
        }

        let mut size = Position(0, 0);
        if let Some(first_line) = pixels.lines().next() {
            size.1 = first_line.trim().chars().count();
        }
        else {
            return Err(ParseImageError::MissingImage);
        }

        let pixels = pixels.trim().lines()
            .flat_map(|s| {
                s.trim().chars().map(|c| {
                    Pixel::from_char(c).ok_or(ParseImageError::InvalidPixel(c))
                })
            })
            .collect::<Result<Vec<Pixel>, ParseImageError>>()?;

        // IMPROVEMENT: check for InconsistentRowLength before collecting
        size.0 = pixels.len() / size.1;
        if size.0 * size.1 != pixels.len() { return Err(ParseImageError::IncosistentRowLength); }
        let buffer = Vec::new();

        Ok(Image {
            pixels, buffer, algorithm, size, padding_pixel: Pixel::Dark
        })
    }
}
impl Image {
    fn count_light_pixels(&self) -> usize {
        self.pixels.iter().filter(|&&pixel| pixel == Pixel::Light).count()
    }

    fn enhance(&mut self) {
        self.buffer.clear();
        if let Some(additional) = (self.pixels.len() + 2*(self.size.0 + self.size.1) + 4).checked_sub(self.buffer.capacity()) {
            self.buffer.reserve(additional);
        }
        let padding_bit = if self.padding_pixel == Pixel::Light { 1 } else { 0 };

        // generate the pixels of the new enhanced image row-by-row, from left to right
        let mut idx;    // index into the algorithm look-up table
        for i in 0..(self.size.0+2) {   // iterate over the rows of the new image
            idx = if padding_bit == 1 { 0b111111111 } else { 0 };
            for j in 0..self.size.1 {   // iterate over the columns of the new image (ignoring the last two columns)
                idx <<= 1;
                idx &= 0b110110110;

                // out-of-bounds pixels will be handled in the else block (using `padding_bit`)
                // ! The bottom two pixels need extra checks to check if they are within the lower bounds.
                //   Instead of doing `checked_sub`, since the index is guaranteed to be positive (note
                //   that this is not true for pixels outside the lower bound, thus we need to check `i`
                //   first for that case), we can just try to `get` the pixel. If the pixels is not found,
                //   then it is out of bounds.

                // fix upper-right bit
                if i > 1 {
                    if self.pixels[self.size.1 * (i-2) + j] == Pixel::Light {
                        idx |= 0b1000000;
                    }
                } else { idx |= padding_bit << 6; }

                // fix right bit
                let pixel_get = if i > 0 {
                    self.pixels.get(self.size.1 * (i-1) + j)
                } else { None };
                if let Some(pixel) = pixel_get {
                    if *pixel == Pixel::Light {
                        idx |= 0b1000;
                    }
                } else { idx |= padding_bit << 3; }

                // fix lower-right bit
                if let Some(pixel) = self.pixels.get(self.size.1 * i + j) {
                    if *pixel == Pixel::Light {
                        idx |= 0b1;
                    }
                } else { idx  |= padding_bit; }

                self.buffer.push(self.algorithm[idx]);
            }

            // the two right-most pixels only need to see the padding bits
            idx = (idx << 1) & 0b110110110;
            if padding_bit == 1 { idx |= 0b001001001; }
            self.buffer.push(self.algorithm[idx]);

            idx = (idx << 1) & 0b110110110;
            if padding_bit == 1 { idx |= 0b001001001; }
            self.buffer.push(self.algorithm[idx]);
        }

        self.size.0 += 2;
        self.size.1 += 2;
        mem::swap(&mut self.buffer, &mut self.pixels);

        match self.padding_pixel {
            Pixel::Light => self.padding_pixel = self.algorithm[0b111111111],
            Pixel::Dark => self.padding_pixel = self.algorithm[0b000000000],
        }
    }
}
impl Display for Image {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for (i, pixel) in self.pixels.iter().enumerate() {
            write!(f, "{}", pixel)?;
            if (i+1) % (self.size.1) == 0 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}




pub fn day20_main(file_data: &str) -> (usize, usize) {
    let mut image = file_data.parse::<Image>()
        .unwrap_or_else(|e| {
            panic!("error parsing the provided image: {}", e);
        });

    // Part 1
    image.enhance();
    image.enhance();
    let part1_ans = image.count_light_pixels();

    // Part 2
    for _ in 0..48 {
        image.enhance();
    }
    let part2_ans = image.count_light_pixels();


    println!("[Part 1] After enhancing the image twice, the image has {} lit pixels.", part1_ans);
    println!("[Part 1] After enhancing the image 50 times, the image has {} lit pixels.", part2_ans);

    (part1_ans, part2_ans)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let test_data =
            "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

            #..#.
            #....
            ##..#
            ..#..
            ..###";
        println!("{}", test_data.parse::<Image>().unwrap());
        assert_eq!(day20_main(test_data), (35, 3351));
    }
}