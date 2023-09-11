use learn::greet;
use std::collections::HashSet;
use std::collections::HashMap;

fn main() {
    let (mut bunnies , mut carrots) = (1,2);
    const FOO_BAR:isize = 10;
    {
        let mut bunnies = 12;
        println!("{}   {}",bunnies, FOO_BAR );
    }
     println!("{}    {}",bunnies, FOO_BAR );
     greet();
     three_sum(vec![-1,0,1,2,-1,-4])
         .iter()
         .for_each(|inner_vec|{
            print!("[ "); 
            inner_vec.iter().for_each(|&value| print!("{} ", value));
            print!("]\n");
         });

}


pub fn is_subsequence(s: String, t: String) -> bool {
    let (a1, a2):(Vec<char>, Vec<char>) = (s.chars().collect(), t.chars().collect());
    let (mut l, mut r) = (0, 0);
    while l < a1.len() && r < a2.len() {
        if a1[l] == a2[r] {
            l+=1;
        }
        r+=1;
    }
    l == a1.len()
}

pub fn three_sum(mut nums: Vec<i32>) ->Vec<Vec<i32>> {
    if nums.len() < 3 {return vec![]}
    nums.sort();
    let mut set = HashSet::new();

    for i in 0..nums.len() {
        let (mut j, mut k) = (i+1, nums.len()-1);
        while j < k {
            match nums[i] + nums[j] + nums[k]{
                s if s > 0 => k-=1,
                s if s < 0 => j+=1,
                _  => {
                    set.insert((nums[i], nums[j], nums[k]));
                    j+=1;
                    while nums[j] == nums[j-1] && j < k{j+=1;}
                }
            }
        }
    }
    set.iter().map(|i| vec![i.0, i.1, i.2]).collect()
}


pub fn three_sum_optimal(mut nums:Vec<i32>)-> Vec<Vec<i32>> {
    nums.sort();
    let mut res:Vec<Vec<i32>> = vec![];

    for i in 0..nums.len(){
        if i > 0 && nums[i] == nums[i-1] {continue;}
        let (mut j, mut k) = (i+1, nums.len()-1);
        while j < k {
            match nums[i] + nums[j] + nums[k] {
                s if s < 0 => j+=1,
                s if s > 0 => k-=1,
                _ => {
                    res.push(vec![nums[i], nums[j], nums[k]]);
                    j+=1;
                    while nums[j] == nums[j-1] && j < k {j+=1;}
                }
            }
        }
    }
    res
}

pub fn two_sum(nums:Vec<i32>, target:i32) -> Vec<i32> {
    
}
