use super::index_iter::IndexIter;
use std::ops::Add;

/// A symmetry of the group D4.
///
/// The clockwise rotation is performed first, and then the horizontal flip.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Symmetry {
    rotation_cw: u8,
    flipped_horiz: bool,
}

impl Symmetry {
    pub const IDENTITY: Self = Self::new(0, false);
    pub const ROT_CW: Self = Self::new(1, false);
    pub const ROT_180: Self = Self::new(2, false);
    pub const ROT_CCW: Self = Self::new(3, false);
    pub const FLIP_HORIZ: Self = Self::new(0, true);
    pub const FLIP_DOWN_LEFT: Self = Self::new(1, true);
    pub const FLIP_VERT: Self = Self::new(2, true);
    pub const FLIP_DOWN_RIGHT: Self = Self::new(3, true);

    pub const ALL: [Self; 8] = [
        Self::IDENTITY,
        Self::ROT_CW,
        Self::ROT_180,
        Self::ROT_CCW,
        Self::FLIP_HORIZ,
        Self::FLIP_DOWN_LEFT,
        Self::FLIP_VERT,
        Self::FLIP_DOWN_RIGHT,
    ];

    const fn new(rotation_cw: u8, flipped_horiz: bool) -> Self {
        Self {
            rotation_cw,
            flipped_horiz,
        }
    }

    pub fn dimensions(self, width: usize, height: usize) -> (usize, usize) {
        if self.rotation_cw % 2 == 0 {
            (width, height)
        } else {
            (height, width)
        }
    }

    pub fn indices(self, width: usize, height: usize) -> IndexIter {
        let horiz_end = width - 1;
        let vert_end = height - 1;
        let (horiz_start, horiz_end, vert_start, vert_end) = match self.rotation_cw {
            0 => (0, horiz_end, 0, vert_end),
            1 => (0, vert_end, horiz_end, 0),
            2 => (horiz_end, 0, vert_end, 0),
            3 => (vert_end, 0, 0, horiz_end),
            _ => unreachable!(),
        };

        let (horiz_start, horiz_end) = if !self.flipped_horiz {
            (horiz_start, horiz_end)
        } else {
            (horiz_end, horiz_start)
        };
        let is_flipped = self.rotation_cw % 2 != 0;

        IndexIter::new(horiz_start, horiz_end, vert_start, vert_end, is_flipped)
    }
}

impl Add for Symmetry {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let rotation_cw = if self.flipped_horiz {
            (4 + self.rotation_cw - other.rotation_cw) % 4
        } else {
            (self.rotation_cw + other.rotation_cw) % 4
        };

        let flipped_horiz = self.flipped_horiz ^ other.flipped_horiz;
        Self {
            rotation_cw,
            flipped_horiz,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Symmetry;

    #[test]
    fn symmetries_add_rotation_correctly_when_flipped() {
        assert_eq!(
            Symmetry::ROT_CCW + Symmetry::FLIP_HORIZ,
            Symmetry::FLIP_DOWN_RIGHT
        );
    }

    #[test]
    fn flipped_symmetries_add_rotation_correctly() {
        assert_eq!(
            Symmetry::FLIP_HORIZ + Symmetry::ROT_CW,
            Symmetry::FLIP_DOWN_RIGHT
        );
        assert_eq!(
            Symmetry::FLIP_DOWN_RIGHT + Symmetry::ROT_CW,
            Symmetry::FLIP_VERT
        );
        assert_eq!(
            Symmetry::FLIP_VERT + Symmetry::ROT_CW,
            Symmetry::FLIP_DOWN_LEFT
        );
        assert_eq!(
            Symmetry::FLIP_DOWN_LEFT + Symmetry::ROT_CW,
            Symmetry::FLIP_HORIZ
        );
    }

    #[test]
    fn flipping_reverses_rotation() {
        assert_eq!(
            Symmetry::FLIP_HORIZ + Symmetry::ROT_CW + Symmetry::FLIP_HORIZ,
            Symmetry::ROT_CCW
        );
    }

    #[test]
    fn identity_indices_iterate_correctly() {
        let actual = Symmetry::IDENTITY.indices(3, 3).collect::<Vec<_>>();
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

    #[test]
    fn rot_cw_indices_iterate_correctly() {
        let actual = Symmetry::ROT_CW.indices(3, 3).collect::<Vec<_>>();
        let expected = vec![
            (0, 2),
            (0, 1),
            (0, 0),
            (1, 2),
            (1, 1),
            (1, 0),
            (2, 2),
            (2, 1),
            (2, 0),
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn rot_180_indices_iterate_correctly() {
        let actual = Symmetry::ROT_180.indices(3, 3).collect::<Vec<_>>();
        let expected = vec![
            (2, 2),
            (1, 2),
            (0, 2),
            (2, 1),
            (1, 1),
            (0, 1),
            (2, 0),
            (1, 0),
            (0, 0),
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn flip_horiz_indices_iterate_correctly() {
        let actual = Symmetry::FLIP_HORIZ.indices(3, 3).collect::<Vec<_>>();
        let expected = vec![
            (2, 0),
            (1, 0),
            (0, 0),
            (2, 1),
            (1, 1),
            (0, 1),
            (2, 2),
            (1, 2),
            (0, 2),
        ];
        assert_eq!(actual, expected);
    }
}
