use std::{
    collections::VecDeque,
    env,
    fs::File,
    io::Read,
    ops::{RangeInclusive, Shr},
};

const SCREEN_WIDTH: u8 = 7;

#[derive(Default, Clone, Copy)]
struct ScreenLine(u8);

impl std::fmt::Debug for ScreenLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in (0..SCREEN_WIDTH).rev() {
            let mask = 1 << i;
            let pixel = if self.0 & mask != 0 { '#' } else { '.' };
            write!(f, "{}", pixel)?;
        }
        Ok(())
    }
}

impl std::ops::BitAnd for ScreenLine {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl std::ops::BitOr for ScreenLine {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl std::ops::Shl<u8> for ScreenLine {
    type Output = Self;
    fn shl(self, rhs: u8) -> Self::Output {
        Self(self.0 << rhs)
    }
}

impl std::ops::Shr<u8> for ScreenLine {
    type Output = Self;
    fn shr(self, rhs: u8) -> Self::Output {
        Self(self.0 >> rhs)
    }
}

impl ScreenLine {
    fn can_shl(&self) -> bool {
        self.0 & (1 << (SCREEN_WIDTH - 1)) == 0
    }

    fn can_shr(&self) -> bool {
        self.0 & 1 == 0
    }

    fn intersect(&self, other: &Self) -> bool {
        self.0 & other.0 != 0
    }
}

#[derive(Default, Clone)]
struct Sprite {
    y_position: usize,
    mask: VecDeque<ScreenLine>,
}

impl std::fmt::Debug for Sprite {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, l) in self.mask.iter().enumerate() {
            if i != 0 {
                writeln!(f)?;
            }
            write!(f, "{:?}", l)?;
        }
        Ok(())
    }
}

impl Sprite {
    fn from_preset(id: usize, floor_y_position: usize) -> Self {
        let mut mask = VecDeque::default();
        match id {
            0 => {
                mask.push_back(ScreenLine(0b1111000));
            }
            1 => {
                mask.push_back(ScreenLine(0b0100000));
                mask.push_back(ScreenLine(0b1110000));
                mask.push_back(ScreenLine(0b0100000));
            }
            2 => {
                mask.push_back(ScreenLine(0b0010000));
                mask.push_back(ScreenLine(0b0010000));
                mask.push_back(ScreenLine(0b1110000));
            }
            3 => {
                mask.push_back(ScreenLine(0b1000000));
                mask.push_back(ScreenLine(0b1000000));
                mask.push_back(ScreenLine(0b1000000));
                mask.push_back(ScreenLine(0b1000000));
            }
            4 => {
                mask.push_back(ScreenLine(0b1100000));
                mask.push_back(ScreenLine(0b1100000));
            }
            _ => panic!("unknown"),
        }
        let y_position = floor_y_position + 3 + mask.len();
        mask.iter_mut().for_each(|l| *l = *l >> 2);
        Sprite { y_position, mask }
    }

    fn move_by(&mut self, b: u8) -> Self {
        match b {
            b'<' => self.move_left(),
            b'>' => self.move_right(),
            _ => self.clone(),
        }
    }

    fn move_left(&self) -> Self {
        if !self.mask.iter().all(ScreenLine::can_shl) {
            return self.clone();
        }
        Self {
            y_position: self.y_position,
            mask: self.mask.iter().map(|l| *l << 1).collect(),
        }
    }

    fn move_right(&self) -> Self {
        if !self.mask.iter().all(ScreenLine::can_shr) {
            return self.clone();
        }
        Self {
            y_position: self.y_position,
            mask: self.mask.iter().map(|l| *l >> 1).collect(),
        }
    }

    fn move_down(&self) -> Self {
        Self {
            y_position: self.y_position - 1,
            mask: self.mask.clone(),
        }
    }

    fn y_bottom(&self) -> usize {
        self.y_position - (self.mask.len() - 1)
    }

    fn intersect(&self, other: &Self) -> bool {
        self.shared_lines(other)
            .map(|mut shared| shared.any(|(l, r)| l.intersect(r)))
            .unwrap_or(false)
    }

    fn shared_line_range(
        &self,
        other: &Self,
    ) -> Option<(RangeInclusive<usize>, RangeInclusive<usize>)> {
        let self_bottom = self.y_bottom();
        let other_bottom = other.y_bottom();
        if self_bottom > other.y_position || other_bottom > self.y_position {
            return None;
        }
        let shared_top = self.y_position.min(other.y_position);
        let shared_bottom = self_bottom.max(other_bottom);
        if shared_top < shared_bottom {
            return None;
        }

        let self_shared_range =
            self.y_position.abs_diff(shared_top)..=self.y_position.abs_diff(shared_bottom);
        let other_shared_range =
            other.y_position.abs_diff(shared_top)..=other.y_position.abs_diff(shared_bottom);

        Some((self_shared_range, other_shared_range))
    }

    fn shared_lines<'a>(
        &'a self,
        other: &'a Self,
    ) -> Option<impl Iterator<Item = (&'a ScreenLine, &'a ScreenLine)>> {
        self.shared_line_range(other).map(|(r1, r2)| {
            let self_shared_lines = self.mask.range(r1);
            let other_shared_lines = other.mask.range(r2);
            self_shared_lines.zip(other_shared_lines)
        })
    }

    fn shared_lines_mut<'a>(
        &'a mut self,
        other: &'a mut Self,
    ) -> Option<impl Iterator<Item = (&'a mut ScreenLine, &'a mut ScreenLine)>> {
        self.shared_line_range(other).map(|(r1, r2)| {
            let self_shared_lines = self.mask.range_mut(r1);
            let other_shared_lines = other.mask.range_mut(r2);
            self_shared_lines.zip(other_shared_lines)
        })
    }
}

fn main() {
    let fpath = env::args()
        .nth(1)
        .expect("Path to input file is not given!");

    let mut buf = Vec::default();
    let mut file = File::open(fpath).unwrap();
    file.read_to_end(&mut buf).unwrap();

    let mut floor_mask = VecDeque::default();
    floor_mask.push_front(ScreenLine(0b1111111));
    let mut floor = Sprite {
        mask: floor_mask,
        y_position: 0,
    };

    let mut rocks_count: usize = 0;
    let mut sprite = Sprite::from_preset(rocks_count % 5, floor.y_position);

    for b in buf.iter().cycle() {
        let sprite_moved = sprite.move_by(*b);
        if !sprite_moved.intersect(&floor) {
            sprite = sprite_moved;
        }

        let sprite_moved = sprite.move_down();
        if !sprite_moved.intersect(&floor) {
            sprite = sprite_moved;
            continue;
        }

        if let Some(shared) = floor.shared_lines_mut(&mut sprite) {
            shared.for_each(|(l, r)| *l = *l | *r);
        }
        if sprite.y_position > floor.y_position {
            let n = sprite.mask.len().min(sprite.y_position - floor.y_position);
            floor.y_position += n;
            for l in sprite.mask.range(0..n).rev() {
                floor.mask.push_front(*l);
            }
        }

        rocks_count += 1;
        if rocks_count == 2022 {
            break;
        }
        sprite = Sprite::from_preset(rocks_count % 5, floor.y_position);
    }

    println!("new floor");
    println!("{:?}", floor);
    println!();
    println!("{}", floor.y_position);
}
