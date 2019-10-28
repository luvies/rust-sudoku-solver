use serde::{Deserialize, Serialize};
use std::fmt;
use std::mem::{self, MaybeUninit};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Serializable {
    grid: [[Option<u8>; 9]; 9],
}

impl Serializable {
    pub fn new() -> Self {
        Serializable {
            grid: [[None; 9]; 9],
        }
    }
}

impl From<Working> for Serializable {
    fn from(item: Working) -> Self {
        let mut szl = Serializable::new();

        for (x, col) in item.grid.iter().enumerate() {
            for (y, item) in col.iter().enumerate() {
                match item {
                    Item::Possible(_) => szl.grid[x][y] = None,
                    Item::Value(val) => szl.grid[x][y] = Some(*val),
                }
            }
        }

        szl
    }
}

#[derive(Debug, Clone)]
enum Item {
    Possible(Vec<u8>),
    Value(u8),
}

impl Item {
    pub fn possible() -> Item {
        Item::Possible(vec![1, 2, 3, 4, 5, 6, 7, 8, 9])
    }
}

#[derive(Debug, Clone, Copy)]
struct ItemPoint {
    x: usize,
    y: usize,
}

impl fmt::Display for ItemPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

#[derive(Debug, Clone)]
pub struct Working {
    grid: [[Item; 9]; 9],
}

fn iter_points<F>(mut f: F)
where
    F: FnMut(ItemPoint) -> (),
{
    for x in 0..9 {
        for y in 0..9 {
            f(ItemPoint { x, y });
        }
    }
}

impl Working {
    pub fn new() -> Self {
        let mut grid: [[MaybeUninit<Item>; 9]; 9] = unsafe { MaybeUninit::uninit().assume_init() };

        for col in &mut grid {
            for item in &mut *col {
                *item = MaybeUninit::new(Item::possible());
            }
        }

        Working {
            grid: unsafe { mem::transmute(grid) },
        }
    }

    pub fn solve(&mut self) -> bool {
        iter_points(|point| {
            if let Item::Value(val) = self.grid[point.x][point.y] {
                self.carry_found(val, point);
            }
        });

        while self.do_solve() {}

        let mut all_solved = true;
        iter_points(|ItemPoint { x, y }| {
            if let Item::Possible(_) = self.grid[x][y] {
                all_solved = false;
            }
        });

        all_solved
    }

    fn do_solve(&mut self) -> bool {
        let mut changes = false;
        iter_points(|point| changes |= self.handle_point(point));
        changes
    }

    fn handle_point(&mut self, point: ItemPoint) -> bool {
        let ItemPoint { x, y } = point;

        if let Item::Possible(ref mut poss) = self.grid[x][y] {
            if poss.len() == 1 {
                let val = *poss.first().unwrap();
                self.grid[x][y] = Item::Value(val);
                self.carry_found(val, point);
                return true;
            }
        }

        false
    }

    fn carry_found(&mut self, val: u8, point: ItemPoint) {
        let ItemPoint { x, y } = point;

        let mut carry = |x: usize, y: usize| {
            if let Item::Possible(ref mut poss) = self.grid[x][y] {
                poss.retain(|&cur| cur != val);
            }
        };

        for yc in (0..9).filter(|&cy| cy != y) {
            carry(x, yc);
        }

        for xc in (0..9).filter(|&cx| cx != x) {
            carry(xc, y);
        }
    }
}

impl From<Serializable> for Working {
    fn from(item: Serializable) -> Self {
        let mut work = Working::new();

        for (x, col) in item.grid.iter().enumerate() {
            for (y, item) in col.iter().enumerate() {
                if let Some(val) = item {
                    work.grid[x][y] = Item::Value(*val);
                }
            }
        }

        work
    }
}

const H_SPLIT: &str = "-------------";

impl fmt::Display for Working {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut out: String = "".to_owned();

        for (x, col) in self.grid.iter().enumerate() {
            if x % 3 == 0 {
                out = format!("{}{}\n", out, H_SPLIT);
            }

            for (y, item) in col.iter().enumerate() {
                if y % 3 == 0 {
                    out = format!("{}{}", out, "|");
                }

                out = format!(
                    "{}{}",
                    out,
                    match item {
                        Item::Possible(_) => "?".to_owned(),
                        Item::Value(val) => val.to_string(),
                    }
                );
            }

            out = format!("{}{}", out, "|\n");
        }

        write!(f, "{}{}", out, H_SPLIT)
    }
}
