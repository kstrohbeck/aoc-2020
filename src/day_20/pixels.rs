use super::{index_iter::IndexIter, symmetry::Symmetry};
use std::{
    fmt,
    ops::{Index, IndexMut},
};

/// Two-dimensional array of 1-bit pixels.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pixels {
    width: usize,
    height: usize,
    pixels: Vec<bool>,
}

impl Pixels {
    /// Create an empty image of the given dimensions.
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![false; width * height],
        }
    }

    /// Create an image from the given dimensions and pixels.
    ///
    /// Returns `None` if the pixel vector is not the correct size.
    pub fn from_raw(width: usize, height: usize, pixels: Vec<bool>) -> Option<Self> {
        if pixels.len() == width * height {
            Some(Self {
                width,
                height,
                pixels,
            })
        } else {
            None
        }
    }

    /// Returns the width of the image.
    pub fn width(&self) -> usize {
        self.width
    }

    /// Returns the height of the image.
    pub fn height(&self) -> usize {
        self.height
    }

    /// Get a reference to the pixel at the given coordinates.
    pub fn get(&self, horiz: usize, vert: usize) -> Option<&bool> {
        let idx = self.coord_to_pixel_index(horiz, vert)?;
        self.pixels.get(idx)
    }

    /// Get a mutable reference to the pixel at the given coordinates.
    pub fn get_mut(&mut self, horiz: usize, vert: usize) -> Option<&mut bool> {
        let idx = self.coord_to_pixel_index(horiz, vert)?;
        self.pixels.get_mut(idx)
    }

    fn coord_to_pixel_index(&self, horiz: usize, vert: usize) -> Option<usize> {
        guard(horiz < self.width && vert < self.height)?;
        Some(vert * self.width + horiz)
    }

    fn pixel_index_to_coord(&self, idx: usize) -> Option<(usize, usize)> {
        let horiz = idx % self.width;
        let vert = idx / self.width;
        guard(vert < self.height)?;
        Some((horiz, vert))
    }

    /// Iterator over the indices of the pixels.
    pub fn indices(&self) -> IndexIter {
        IndexIter::new(0, self.width, 0, self.height, false)
    }

    /// Iterator over references to pixels and their indices.
    pub fn pixels(&self) -> impl Iterator<Item = &bool> {
        self.pixels.iter()
    }

    /// Iterator over mutable references to pixels and their indices.
    pub fn pixels_mut(&mut self) -> impl Iterator<Item = &mut bool> {
        self.pixels.iter_mut()
    }

    /// Returns a region of the `Pixels` defined by the `Rect`.
    pub fn region(&self, rect: Rect) -> Self {
        let mut new = Self::new(rect.width, rect.height);

        let old_iter = rect.indices().map(|i| self[i]);
        let new_iter = new.pixels_mut();

        for (o, n) in old_iter.zip(new_iter) {
            *n = o;
        }

        new
    }

    /// Returns a copy of this image with the given image blitted onto a portion of it.
    pub fn blit(&self, src: &Self, horiz: usize, vert: usize) -> Self {
        unimplemented!()
    }

    /// Returns a copy of the image rotated or flipped according to the given symmetry.
    pub fn apply_symmetry(&self, sym: Symmetry) -> Self {
        let (width, height) = sym.dimensions(self.width, self.height);
        let mut new = Self::new(width, height);

        let old_iter = sym.indices(self.width, self.height).map(|i| self[i]);
        let new_iter = new.pixels_mut();

        for (o, n) in old_iter.zip(new_iter) {
            *n = o;
        }

        new
    }
}

impl Index<(usize, usize)> for Pixels {
    type Output = bool;

    fn index(&self, (h, v): (usize, usize)) -> &Self::Output {
        self.get(h, v).unwrap()
    }
}

impl IndexMut<(usize, usize)> for Pixels {
    fn index_mut(&mut self, (h, v): (usize, usize)) -> &mut Self::Output {
        self.get_mut(h, v).unwrap()
    }
}

impl fmt::Display for Pixels {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: Implement.
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Rect {
    pub horiz: usize,
    pub vert: usize,
    pub width: usize,
    pub height: usize,
}

impl Rect {
    pub fn indices(self) -> IndexIter {
        IndexIter::new(
            self.horiz,
            self.horiz + self.width - 1,
            self.vert,
            self.vert + self.width - 1,
            false,
        )
    }
}

fn guard(cond: bool) -> Option<()> {
    if cond {
        Some(())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::{Pixels, Rect, Symmetry};

    #[test]
    fn full_region_returns_identical_pixels() {
        let pixels = Pixels::from_raw(
            3,
            3,
            vec![true, true, true, false, false, false, false, false, false],
        )
        .unwrap();
        let rect = Rect {
            horiz: 0,
            vert: 0,
            width: pixels.width(),
            height: pixels.height(),
        };
        let actual = pixels.region(rect);
        assert_eq!(actual, pixels);
    }

    #[test]
    fn region_crops() {
        let pixels = Pixels::from_raw(
            3,
            3,
            vec![true, true, true, false, false, false, false, false, false],
        )
        .unwrap();
        let rect = Rect {
            horiz: 0,
            vert: 0,
            width: 2,
            height: 2,
        };
        let expected = Pixels::from_raw(2, 2, vec![true, true, false, false]).unwrap();
        let actual = pixels.region(rect);
        assert_eq!(actual, expected);
    }

    #[test]
    fn applying_rotation_creates_correct_pixels() {
        let pixels = Pixels::from_raw(
            3,
            3,
            vec![true, true, true, false, false, false, false, false, false],
        )
        .unwrap();
        let expected = Pixels::from_raw(
            3,
            3,
            vec![false, false, true, false, false, true, false, false, true],
        )
        .unwrap();
        let actual = pixels.apply_symmetry(Symmetry::ROT_CW);
        assert_eq!(actual, expected);
    }

    #[test]
    fn applying_flip_creates_correct_pixels() {
        let pixels = Pixels::from_raw(
            3,
            3,
            vec![false, false, true, false, false, true, false, false, true],
        )
        .unwrap();
        let expected = Pixels::from_raw(
            3,
            3,
            vec![true, false, false, true, false, false, true, false, false],
        )
        .unwrap();
        let actual = pixels.apply_symmetry(Symmetry::FLIP_HORIZ);
        assert_eq!(actual, expected);
    }

    #[test]
    fn rect_iterates_correctly() {
        let rect = Rect {
            horiz: 0,
            vert: 0,
            width: 3,
            height: 3,
        };
        let actual = rect.indices().collect::<Vec<_>>();
        let expected = vec![
            (0, 0),
            (1, 0),
            (2, 0),
            (0, 1),
            (1, 1),
            (2, 1),
            (0, 2),
            (1, 2),
            (2, 2),
        ];
        assert_eq!(actual, expected);
    }
}
