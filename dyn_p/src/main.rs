/// DP's 0/1 Knapsack Problem
/// 

// cap = 6
// item  (weight, relevance)
// water, 3, 10
// book, 1, 3
// food, 2, 9
// coat, 2, 5
// cam, 1, 6

type WeightRelevance = (usize, usize);
type Item = (String, WeightRelevance);
type Memo = Vec<Vec<Item>>;

const SIZE: usize = 6;

fn main() {
    let mut items: [Item; 5] = [
        ("water".to_string(), (3, 10)),
        ("book".to_string(), (1, 3)),
        ("food".to_string(), (2, 9)),
        ("coat".to_string(), (2, 5)),
        ("cam".to_string(), (1, 6)),
    ];

    // memo is key, it is responsible for cache and store the best values
    // it is a contiguous array where each bucket has a collection (vec) of items which are already on the slot (ideally) (it can be null, len = 0)
    let mut memo: Memo = vec![vec![]; SIZE];

    // items.sort_by(relevance);
    resolve(&mut items, &mut memo);
    // dbg!(memo);
}

/// This memo contiguous array already has all information we need, 
/// now it is just iterate over it on a reverse order in order to find the best option of each level 
/// and use cache of already calculated layers
/// 
fn resolve(items: &mut [Item; 5], memo: &mut Memo) {
    // weight iteration
    // O(n) | n :: weight options qntty
    for w in 1..=6 {
        println!("BAG SIZE: {}", w);
        // iterating reversely means the better option for that level is the first one we choose
        // so maybe it should just return if it's value is greater and fit (?)
        //   :: even fitting and being the highest one, doesnt means it's relation is the best for the bucket
        // O(j) | j :: items.len()
        for item in items.iter() {
            // (weight / relevance)
            let (item_w, item_r) = item.1;

            // the item fits on this level
            // and if it fits, we check if adding (it + (better solution of remaning space)) > the value already set there (memo[idx])
            // so this check should be if item_w <= w :: it fits
            if item_w <= w {
                if memo[w - 1].len() == 0 {
                    memo[w - 1] = vec![item.clone()];
                }

                let ((better_weight, better_rel), mut better_its) = better_of(w - 1, w - item_w, &memo);

                let doesnt_overflow = (item_w + better_weight) <= w;

                // should check for false array .push here too (push new already added items or when item doesnt fit)
                if (item_r + better_rel) > total_relevance_weight(&memo[w - 1]).1 && doesnt_overflow {
                    // should push item into Memo with it's better pair(s)
                    // should check if item isn't already on the better_of returned slot
                    if !better_its.contains(&item) {
                        better_its.push(item.clone());
                    }

                    memo[w - 1] = better_its
                }
            }
            println!("Finished {}, going to next item", item.0);
            dbg_memo(&memo);
        }
    }
}

fn dbg_memo(memo: &Memo) {
    for (idx, slot) in memo.iter().enumerate() {
        if slot.len() == 0 { continue }

        print!("[SLOT {}] => ", idx);

        for it in slot.iter() {
            println!("[{} {:?}]", it.0, it.1);
        }
        println!("\n");
    };
    println!("==========")
}

/// Get better options of remaining slot
/// 
fn better_of(slot_idx: usize, remaining_space: usize, memo: &Memo) -> ((usize, usize), Vec<Item>) {
    if remaining_space == 0 { return ((0, 0), vec![]) };

    // -1 because memo's idx is 0 based
    let rel = total_relevance_weight(&memo[slot_idx]);
    (rel, memo[slot_idx].clone())
}

/// Calculate relevance given a set of Item
/// 
fn total_relevance_weight(items: &Vec<Item>) -> (usize, usize) {
    let rel = items.iter().fold(0, |acc, item| {
        acc + item.1.1
    });

    let weight = items.iter().fold(0, |acc, item| {
        acc + item.1.0
    });

    (weight, rel)
}

// fn map_vec(v: &mut Vec<Item>, items: &[Item; 5], limit: usize) {
//     // here I higly base the algorithm on the sort of each row, i'll use memoization here prob.
//     // I sort by relevance so I make sure no other option is better for that
//     // instead of just joining the two solution, I will take the better option of (options[i] -1) + options[i] or just options[i - 1], if options [i-1] is still better

//     // Function f :: f(x) -> y :: y = better option for that level, which can be used recursivelly, retiring a index of the call (if needed)
//     // lets say, betterof(1);

//     // take level-specific items (this calc is only made 1 time)
//     // sort it based of relevance, so [0] is the most, etc..
//     v.sort_by(relevance);
// }

// fn relevance<'a, 'b>(a: &'a Item, b: &'b Item) -> Ordering {}