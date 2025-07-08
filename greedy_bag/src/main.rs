/// An optimal solution of a basic greedy algorithm
/// 

#[derive(Debug)]
struct Item {
    id: u16,
    scr: f32,
    wgt: f32,
}

impl Item {
    fn as_ratio(&self) {
        println!("{}", self.scr / self.wgt);
    }

    fn ratio(&self) -> f32 {
        self.scr / self.wgt
    }
}


/// Default bag size
/// 
const SIZE: usize = 5;

/// Weight limit of bag
/// 
const LIMIT: f32 = 25.0;

fn main() {
    let mut bag: [Item; SIZE] = [
        Item { scr: 20.0, wgt: 10.0, id: 1 },
        Item { scr: 30.0, wgt: 10.0, id: 2 },
        Item { scr: 20.0, wgt: 40.0, id: 3 },
        Item { scr: 15.0, wgt: 55.0, id: 4 },
        Item { scr: 12.0, wgt: 25.0, id: 5 },
    ];
    quick_sort(&mut bag);

    for n in bag.iter() { println!("{:?}", n.scr / n.wgt); }

    greed(&mut bag);
}

fn quick_sort(bag: &mut [Item]) {
    /* base case */
    if bag.len() < 2 { return };

    let mut piv_pos = 0;
    let mut curr = 1;

    // ~ O(n + ((piv_pos..curr * n) / 2)) iteration
    while curr < bag.len() {
        dbg!(bag[curr].scr / bag[curr].wgt, bag[piv_pos].scr / bag[piv_pos].wgt);

        if bag[curr].scr / bag[curr].wgt <= bag[piv_pos].scr / bag[piv_pos].wgt {
            l_shift(bag, &mut curr);
            piv_pos += 1;
        }

        curr += 1;
    }

    quick_sort(&mut bag[..piv_pos]);
    quick_sort(&mut bag[piv_pos+1..]);
}

/// Move elements one by one based on pivot position
/// 
fn l_shift<'a>(bag: &'a mut [Item], t_idx: &usize) {
    for i in 0..*t_idx {
        bag.swap(*t_idx-i-1, t_idx-i);

        for it in bag.iter() {
            it.as_ratio();
        }
    };
}

fn greed(bag: &mut [Item]) {
    let mut curr_cost: f32 = 0.0;
    let mut percentages = [0.0; LIMIT as usize];

    for (idx, item) in bag.iter().enumerate() {
        if curr_cost > LIMIT { break; }

        match curr_cost {
            _ if curr_cost + item.ratio() < LIMIT => { curr_cost += item.ratio(); percentages[idx] = 1.0 }
            _ if curr_cost < LIMIT && curr_cost + item.ratio() > LIMIT => {
                let percentage = ((LIMIT - curr_cost) * 100.0) / item.ratio();

                curr_cost += (percentage / 100.0) * item.ratio();
                percentages[idx] = percentage / 100.0;
            }
            _ => ()
        }
    }

    for (idx, perc) in percentages.iter().enumerate() {
        if *perc == 0.0 { continue }

        println!("{} - {}", bag[idx].id, perc);
    }
}