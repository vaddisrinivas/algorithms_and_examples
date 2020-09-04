// greedy solution for activity selection

#[derive(Debug)]
struct arr_elem {
    start_time: i32,
    end_time: i32,
}

fn main() {
    let mut sort_ed: Vec<arr_elem> = Vec::new();
    {
        let mut ip_array = [(12, 25), (10, 20), (20, 30)];
        let mut sorted_arr: Vec<arr_elem> = Vec::new();
        for i in ip_array.iter() {
            if sort_ed.len() == 0 {
                sort_ed.push(arr_elem {
                    start_time: i.0,
                    end_time: i.1,
                });
                continue;
            }
            if sort_ed[0].start_time < i.0 {
                sort_ed.push(arr_elem {
                    start_time: i.0,
                    end_time: i.1,
                })
            } else {
                sort_ed.insert(
                    0,
                    arr_elem {
                        start_time: i.0,
                        end_time: i.1,
                    },
                )
            }
        }
    }
    println!("{:?}", sort_ed);
    let mut count = 1;
    let mut elem = 0;
    let mut cur = 0;
    let mut broke = false;
    //     take the sorted elements and push through
    for j in 1..sort_ed.len() {
        // println!("at {} {}  {:?} {:?}",elem,cur,sort_ed[cur],sort_ed[j]);
        if sort_ed[cur].end_time <= sort_ed[j].start_time {
            println!("yes {:?}", sort_ed[j]);
            count += 1;
            cur = j;
        }
    }

    println!("{}", count);
}
