#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndexIter {
    horiz_start: usize,
    horiz_end: usize,
    vert_start: usize,
    vert_end: usize,
    is_flipped: bool,
    cur_horiz: usize,
    cur_vert: usize,
    is_done: bool,
}

impl IndexIter {
    pub fn new(
        horiz_start: usize,
        horiz_end: usize,
        vert_start: usize,
        vert_end: usize,
        is_flipped: bool,
    ) -> Self {
        Self {
            horiz_start,
            horiz_end,
            vert_start,
            vert_end,
            is_flipped,
            cur_horiz: horiz_start,
            cur_vert: vert_start,
            is_done: false,
        }
    }

    fn step_horiz(&mut self) -> HitEnd {
        step(self.horiz_start, self.horiz_end, &mut self.cur_horiz)
    }

    fn step_vert(&mut self) -> HitEnd {
        step(self.vert_start, self.vert_end, &mut self.cur_vert)
    }

    fn step_horiz_then_vert(&mut self) -> HitEnd {
        if self.step_horiz() == HitEnd::Did {
            self.step_vert()
        } else {
            HitEnd::Didnt
        }
    }

    fn step_vert_then_horiz(&mut self) -> HitEnd {
        if self.step_vert() == HitEnd::Did {
            self.step_horiz()
        } else {
            HitEnd::Didnt
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum HitEnd {
    Did,
    Didnt,
}

fn step(start: usize, end: usize, cur: &mut usize) -> HitEnd {
    if let Some(c) = step_range(start, end, *cur) {
        *cur = c;
        HitEnd::Didnt
    } else {
        *cur = start;
        HitEnd::Did
    }
}

fn step_range(start: usize, end: usize, cur: usize) -> Option<usize> {
    if start < end {
        if cur < end {
            Some(cur + 1)
        } else {
            None
        }
    } else {
        if cur > end {
            Some(cur - 1)
        } else {
            None
        }
    }
}

impl Iterator for IndexIter {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_done {
            return None;
        }

        let cur = (self.cur_horiz, self.cur_vert);

        let hit_end = if !self.is_flipped {
            self.step_horiz_then_vert()
        } else {
            self.step_vert_then_horiz()
        };

        if hit_end == HitEnd::Did {
            self.is_done = true;
        }

        Some(cur)
    }
}
