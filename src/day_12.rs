use nom::{
    branch::alt,
    character::complete::{anychar, digit1},
    combinator::{map, map_opt, map_res},
    sequence::tuple,
    IResult,
};

pub fn star_1(data: String) {
    let actions = parse(&data);
    let mut ship = BasicShip::default();
    for action in actions {
        ship.take_action(action);
    }
    println!("{}", ship.manhattan_distance_from_origin());
}

pub fn star_2(data: String) {
    let actions = parse(&data);
    let mut ship = ShipWithWaypoint::default();
    for action in actions {
        ship.take_action(action);
    }
    println!("{}", ship.manhattan_distance_from_origin());
}

fn parse(data: &str) -> Vec<Action> {
    data.lines()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|s| action(s).unwrap().1)
        .collect()
}

trait Ship {
    fn take_action(&mut self, action: Action);
    fn manhattan_distance_from_origin(&self) -> u32;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct BasicShip {
    facing: Direction,
    horiz_pos: i32,
    vert_pos: i32,
}

impl Default for BasicShip {
    fn default() -> Self {
        Self {
            facing: Direction::East,
            horiz_pos: 0,
            vert_pos: 0,
        }
    }
}

impl Ship for BasicShip {
    fn take_action(&mut self, action: Action) {
        match action {
            Action::Rotate(rot) => {
                self.facing = self.facing.with_rotation(rot);
            }
            Action::Move(dir, amt) => {
                let dir = dir.unwrap_or(self.facing);
                let (horiz, vert) = (dir.horizontal(), dir.vertical());
                self.horiz_pos += horiz * (amt as i32);
                self.vert_pos += vert * (amt as i32);
            }
        }
    }

    fn manhattan_distance_from_origin(&self) -> u32 {
        (self.horiz_pos.abs() + self.vert_pos.abs()) as u32
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ShipWithWaypoint {
    horiz_pos: i32,
    vert_pos: i32,
    waypoint_horiz_offset: i32,
    waypoint_vert_offset: i32,
}

impl Default for ShipWithWaypoint {
    fn default() -> Self {
        Self {
            horiz_pos: 0,
            vert_pos: 0,
            waypoint_horiz_offset: 10,
            waypoint_vert_offset: -1,
        }
    }
}

impl Ship for ShipWithWaypoint {
    fn take_action(&mut self, action: Action) {
        match action {
            Action::Rotate(rot) => {
                let (horiz, vert) = match rot.clockwise_angle() {
                    Angle::Zero => (self.waypoint_horiz_offset, self.waypoint_vert_offset),
                    Angle::Ninety => (-self.waypoint_vert_offset, self.waypoint_horiz_offset),
                    Angle::OneEighty => (-self.waypoint_horiz_offset, -self.waypoint_vert_offset),
                    Angle::TwoSeventy => (self.waypoint_vert_offset, -self.waypoint_horiz_offset),
                };
                self.waypoint_horiz_offset = horiz;
                self.waypoint_vert_offset = vert;
            }
            Action::Move(Some(dir), amt) => {
                self.waypoint_horiz_offset += dir.horizontal() * amt as i32;
                self.waypoint_vert_offset += dir.vertical() * amt as i32;
            }
            Action::Move(None, amt) => {
                self.horiz_pos += self.waypoint_horiz_offset * amt as i32;
                self.vert_pos += self.waypoint_vert_offset * amt as i32;
            }
        }
    }

    fn manhattan_distance_from_origin(&self) -> u32 {
        (self.horiz_pos.abs() + self.vert_pos.abs()) as u32
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Action {
    Rotate(Rotation),
    Move(Option<Direction>, u32),
}

fn action(input: &str) -> IResult<&str, Action> {
    alt((
        map(rotation, Action::Rotate),
        map(movement, |(dir, amt)| Action::Move(dir, amt)),
    ))(input)
}

fn rotation(input: &str) -> IResult<&str, Rotation> {
    let (input, direction) = map_opt(anychar, |c| match c {
        'L' => Some(RotationDirection::CounterClockwise),
        'R' => Some(RotationDirection::Clockwise),
        _ => None,
    })(input)?;
    let (input, angle) = map_opt(u32_, |d| match d {
        0 => Some(Angle::Zero),
        90 => Some(Angle::Ninety),
        180 => Some(Angle::OneEighty),
        270 => Some(Angle::TwoSeventy),
        _ => None,
    })(input)?;
    Ok((input, Rotation { direction, angle }))
}

fn movement(input: &str) -> IResult<&str, (Option<Direction>, u32)> {
    let (input, dir) = map_opt(anychar, |c| match c {
        'N' => Some(Some(Direction::North)),
        'S' => Some(Some(Direction::South)),
        'E' => Some(Some(Direction::East)),
        'W' => Some(Some(Direction::West)),
        'F' => Some(None),
        _ => None,
    })(input)?;
    let (input, amt) = u32_(input)?;
    Ok((input, (dir, amt)))
}

fn u32_(input: &str) -> IResult<&str, u32> {
    map_res(digit1, |s: &str| s.parse::<u32>())(input)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Rotation {
    direction: RotationDirection,
    angle: Angle,
}

impl Rotation {
    fn clockwise_angle(self) -> Angle {
        if self.direction == RotationDirection::Clockwise {
            return self.angle;
        }
        match self.angle {
            Angle::Ninety => Angle::TwoSeventy,
            Angle::TwoSeventy => Angle::Ninety,
            a => a,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Angle {
    Zero,
    Ninety,
    OneEighty,
    TwoSeventy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RotationDirection {
    Clockwise,
    CounterClockwise,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn with_rotation(self, rotation: Rotation) -> Self {
        use Angle::*;
        use Direction::*;
        match (self, rotation.clockwise_angle()) {
            (North, Zero) | (West, Ninety) | (South, OneEighty) | (East, TwoSeventy) => North,
            (East, Zero) | (North, Ninety) | (West, OneEighty) | (South, TwoSeventy) => East,
            (South, Zero) | (East, Ninety) | (North, OneEighty) | (West, TwoSeventy) => South,
            (West, Zero) | (South, Ninety) | (East, OneEighty) | (North, TwoSeventy) => West,
        }
    }

    fn horizontal(self) -> i32 {
        match self {
            Self::West => -1,
            Self::East => 1,
            _ => 0,
        }
    }

    fn vertical(self) -> i32 {
        match self {
            Self::North => -1,
            Self::South => 1,
            _ => 0,
        }
    }
}
