use std::cmp::Ordering;
use std::cmp::Ordering::{Greater, Less};

pub fn manhattan_distance(val: u32) -> i32 {
    let layer = which_layer(val);
    let b = Box::new(layer);
    let pos = b.position(val);

    match pos {
        Some(c) => c.x.abs() + c.y.abs(),
        None => panic!("How did this happen?")
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
enum LayerMatch {
    Yes,
    No(Ordering),
}

struct Box {
    anchor: u32,
    d: u32,
    layer: u32,
    lower_bound: u32,
    upper_bound: u32,
}

#[derive(Debug)]
#[derive(PartialEq)]
struct Coord {
    x: i32,
    y: i32,
}

impl Box {
    fn new(layer: u32) -> Box {
        if layer == 1 {
            return Box {
                anchor: 1,
                d: 1,
                layer: 1,
                lower_bound: 1,
                upper_bound: 1,
            };
        }

        let anchor = ((layer - 1) * (layer - 2) / 2 * 8) + layer;
        let d = layer * 2 - 1;
        let lower_bound = anchor - (layer - 2);

        Box {
            anchor,
            d,
            layer,
            lower_bound,
            upper_bound: lower_bound + (4 * d) - 5,
        }
    }

    fn inside(&self, val: u32) -> LayerMatch {
        if self.layer == 1 {
            if val == 1 {
                return LayerMatch::Yes;
            }

            return LayerMatch::No(Ordering::Greater);
        }

        if self.layer == 2 {
            if val < 10 {
                return LayerMatch::Yes;
            }

            return LayerMatch::No(Ordering::Greater);
        }

        if val >= self.lower_bound && val <= self.upper_bound {
            return LayerMatch::Yes;
        }

        if val < self.lower_bound {
            return LayerMatch::No(Ordering::Less);
        }

        return LayerMatch::No(Ordering::Greater);
    }

    fn position(&self, val: u32) -> Option<Coord> {
        if self.inside(val) != LayerMatch::Yes {
            return None;
        }

        if val == 1 {
            return Some(Coord { x: 0, y: 0 });
        }

        let x: i32;
        let y: i32;
        let tr = self.lower_bound + self.d - 2;
        let tl = tr + self.d - 1;
        let bl = tl + self.d - 1;

        if val <= tr {
            // on the right
            x = self.layer as i32 - 1;
            y = val as i32 - self.anchor as i32;
        } else if val > tr && val <= tl {
            // on the top
            y = self.layer as i32 - 1;
            x = (self.d as i32 / 2 + tr as i32) - val as i32
        } else if val > tl && val <= bl {
            // on the left
            x = self.layer as i32 * -1 + 1;
            y = (self.d as i32 / 2 + tl as i32) - val as i32;
        } else {
            // on the bottom
            y = self.layer as i32 * -1 + 1;
            x = val as i32 - (self.d as i32 / 2 + bl as i32);
        }

        return Some(Coord {
            x,
            y,
        });
    }
}

fn which_layer(val: u32) -> u32 {
    let mut current_layer: u32 = 1;
    let mut upper_bound: u32 = 1;
    let mut lower_bound: u32 = 1;

    loop {
        let b = Box::new(current_layer);
        let ware = b.inside(val);

        if ware == LayerMatch::Yes {
            return current_layer;
        }

        // search for upper bound
        if ware == LayerMatch::No(Greater) && upper_bound == 1 {
            lower_bound = current_layer;
            current_layer *= 2;
            continue;
        }

        // come back down
        if ware == LayerMatch::No(Less) {
            upper_bound = current_layer;
            current_layer = lower_bound + ((upper_bound - lower_bound) / 2);
            continue;
        }

        // go back up
        lower_bound = current_layer;
        current_layer = lower_bound + ((upper_bound - lower_bound) / 2);
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn position_1() {
        let b = Box::new(1);
        assert_eq!(b.position(1).unwrap(), Coord { x: 0, y: 0 });
    }

    #[test]
    fn position_1_not() {
        let b = Box::new(1);
        assert_eq!(b.position(2), None);
    }

    #[test]
    fn position_2() {
        let b = Box::new(2);
        assert_eq!(b.position(2).unwrap(), Coord { x: 1, y: 0 });
    }

    #[test]
    fn position_3() {
        let b = Box::new(2);
        assert_eq!(b.position(3).unwrap(), Coord { x: 1, y: 1 });
    }

    #[test]
    fn position_15() {
        let b = Box::new(3);
        assert_eq!(b.position(15).unwrap(), Coord { x: 0, y: 2 });
    }

    #[test]
    fn position_42() {
        let b = Box::new(4);
        assert_eq!(b.position(42).unwrap(), Coord { x: -3, y: -2 });
    }

    #[test]
    fn position_118() {
        let b = Box::new(6);
        assert_eq!(b.position(118).unwrap(), Coord { x: 2, y: -5 });
    }

    #[test]
    fn which_layer_1() {
        assert_eq!(which_layer(1), 1);
    }

    #[test]
    fn which_layer_5() {
        assert_eq!(which_layer(5), 2);
    }

    #[test]
    fn which_layer_23() {
        assert_eq!(which_layer(23), 3);
    }

    #[test]
    fn which_layer_39() {
        assert_eq!(which_layer(39), 4);
    }

    #[test]
    fn which_layer_77() {
        assert_eq!(which_layer(77), 5);
    }

    #[test]
    fn which_layer_82() {
        assert_eq!(which_layer(82), 6);
    }

    #[test]
    fn which_layer_122() {
        assert_eq!(which_layer(122), 7);
    }

    #[test]
    fn layer_1() {
        let b = Box::new(1);
        assert_eq!(b.inside(1), LayerMatch::Yes);
    }

    #[test]
    fn layer_2() {
        let b = Box::new(2);
        assert_eq!(b.inside(2), LayerMatch::Yes);
    }

    #[test]
    fn layer_11() {
        let b = Box::new(3);
        assert_eq!(b.inside(11), LayerMatch::Yes);
    }

    #[test]
    fn layer_28() {
        let b = Box::new(4);
        assert_eq!(b.inside(28), LayerMatch::Yes);
    }

    #[test]
    fn layer_53() {
        let b = Box::new(5);
        assert_eq!(b.inside(53), LayerMatch::Yes);
    }

    #[test]
    fn layer_12() {
        let b = Box::new(3);
        assert_eq!(b.inside(12), LayerMatch::Yes);
    }

    #[test]
    fn layer_13() {
        let b = Box::new(3);
        assert_eq!(b.inside(13), LayerMatch::Yes);
    }

    #[test]
    fn layer_17() {
        let b = Box::new(3);
        assert_eq!(b.inside(17), LayerMatch::Yes);
    }

    #[test]
    fn layer_21() {
        let b = Box::new(3);
        assert_eq!(b.inside(21), LayerMatch::Yes);
    }

    #[test]
    fn layer_25() {
        let b = Box::new(3);
        assert_eq!(b.inside(25), LayerMatch::Yes);
    }

    #[test]
    fn layer_26() {
        let b = Box::new(3);
        assert_eq!(b.inside(26), LayerMatch::No(Ordering::Greater));
    }

    #[test]
    fn layer_32() {
        let b = Box::new(3);
        assert_eq!(b.inside(32), LayerMatch::No(Ordering::Greater));
    }

    #[test]
    fn layer_38() {
        let b = Box::new(3);
        assert_eq!(b.inside(38), LayerMatch::No(Ordering::Greater));
    }

    #[test]
    fn layer_44() {
        let b = Box::new(3);
        assert_eq!(b.inside(44), LayerMatch::No(Ordering::Greater));
    }

    #[test]
    fn layer_57() {
        let b = Box::new(6);
        assert_eq!(b.inside(57), LayerMatch::No(Ordering::Less));
    }
}