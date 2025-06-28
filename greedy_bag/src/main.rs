/// An optimal solution of a basic greedy algorithm

struct Item {
    scr: f32,
    wgt: f32,
}

/// Default bag size
/// 
const SIZE: usize = 5;

fn main() {
    let bag: [Item; SIZE] = [
        Item { scr: 20.0, wgt: 10.0 },
        Item { scr: 30.0, wgt: 10.0 },
        Item { scr: 20.0, wgt: 40.0 },
        Item { scr: 15.0, wgt: 55.0 },
        Item { scr: 12.0, wgt: 25.0 },
    ];
    let mut ratios = calc_ratio(&bag);

    println!("{:?}", greed(quick_sort(&mut ratios[..])));
}

fn calc_ratio(bag: &[Item; SIZE]) -> [f32; SIZE] {
    let mut ratios: [f32; SIZE] = [0.0; SIZE];

    for (idx, it) in bag.iter().enumerate() {
        ratios[idx] = it.scr / it.wgt
    }

    ratios
}

fn quick_sort(ratio_slice: &mut [f32]) -> &mut [f32] {
    /* base case */
    if ratio_slice.len() < 2 { return ratio_slice };

    let mut piv_pos = 0;
    let mut curr = 1;

    // O(n + (piv_pos..curr * n)) iteration
    while curr < ratio_slice.len() {
        let piv = ratio_slice[piv_pos];

        match curr {
            _ if ratio_slice[curr] <= piv => {
                l_shift(ratio_slice, piv, &mut piv_pos, curr);
            }
            _ => ()
        }

        curr += 1;
    }
    dbg!(&ratio_slice);

    quick_sort(&mut ratio_slice[..piv_pos]);
    quick_sort(&mut ratio_slice[piv_pos+1..]);

    ratio_slice
}

/// Move elements starting from the left of slice
/// 
fn l_shift<'a>(slc: &'a mut [f32], mut piv: f32, piv_pos: &mut usize, t_idx: usize) -> &'a mut [f32] {
    slc[*piv_pos] = slc[t_idx];
    *piv_pos += 1;

    for i in *piv_pos..=t_idx {
        piv = std::mem::replace(&mut slc[i], piv);
    };

    slc
}

fn greed(n: &mut [f32]) -> &mut [f32] {
    dbg!(n);
    todo!();
}