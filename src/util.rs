use std::cmp::{min, max};
use std::collections::BTreeSet;
use std::ops::Sub;

pub fn find_nearest(points: &BTreeSet<(usize, usize)>,
                    my_xy: (usize, usize))
                    -> Option<(usize, usize)> {
    let mut found_xy = None;
    let mut dist = 0;
    for xy in points {
        match found_xy {
            Some(xy) => {
                let newdist = distance(xy, my_xy);
                if newdist < dist {
                    found_xy = Some(xy);
                    dist = newdist;
                }
            }
            None => {
                found_xy = Some(*xy);
                dist = distance(*xy, my_xy);
            }
        }
    }

    return found_xy;
}

// implements Chebyshev distance https://en.wikipedia.org/wiki/Chebyshev_distance
pub fn distance<T>((x1, y1): (T, T), (x2, y2): (T, T)) -> T::Output
    where T: Sub + Ord + Copy,
          <T as Sub>::Output: Ord
{
    let dx = max(x1, x2) - min(x1, x2);
    let dy = max(y1, y2) - min(y1, y2);
    return max(dx, dy);
}
