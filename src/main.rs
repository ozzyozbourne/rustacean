use learn::greet;
use std::collections::HashSet;
use std::collections::HashMap;
use std::vec;

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
    rotate(&mut vec![-1], 2);

    let mut d2: Vec<Vec<i32>> = vec![];
    let arr:Vec<i32> = vec![1,2,3];

    generate_all(&mut d2, &mut vec![], 0, &arr);
    
    for (_,i) in d2.iter().enumerate(){
        for(_,j ) in i.iter().enumerate(){
            print!("{} ", j);
        }println!();
    } 

}

fn generate_all(d2: &mut Vec<Vec<i32>>, d1: &mut Vec<i32>, i: usize, arr: &Vec<i32>) {
    
    if i == arr.len() {
        d2.push(d1.clone());  
        return; 
    }

    d1.push(arr[i]);
    generate_all(d2, d1, i+1, arr);
    _ = d1.pop();
    generate_all(d2, d1, i+1, arr);

}

pub fn two_sum_two(nums:Vec<i32>, target:i32) -> Vec<i32> {
    let (mut left, mut right) = (0, nums.len()-1);
    while left < right {
        match nums[left] + nums[right] {
            s if s < target => left+=1,
            s if s > target => right-=1,
            _  => {left+=1; right+=1; break;}        
        }
    }
    vec![left as i32, right as i32]
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

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut vec:Vec<i32> = vec![];
        let mut map:HashMap<i32, i32> = HashMap::new();
        for (index, value) in nums.iter().enumerate() {
            match map.get(&value) {
                Some(&index_2) =>{vec.append(&mut vec![index as i32, index_2]);}
                None => {map.insert(target - value, index as i32);}
            }
        }
        vec
}

pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    if nums.len() < 4 {return vec![];}    
    nums.sort();
    let (mut res, mut quad):(Vec<Vec<i32>>, Vec<i32>) = (vec![], vec![]);
    
    fn ksum(k:usize, start:usize, target:i64, nums:&mut Vec<i64>, res:&mut Vec<Vec<i32>>, quad:&mut Vec<i32>) {
       if k > 2 {
           for i in start..nums.len()-k+1 {
                if i > start && nums[i] == nums[i-1]{continue;}
                quad.push(nums[i] as i32);
                ksum(k-1, i+1, target-nums[i], nums,res, quad);
                quad.pop();
           }
       }else {
            let (mut l, mut r):(usize, usize) = (start, nums.len()-1);
            while l < r {
                match nums[l] + nums[r] {
                    s if s < target => l+=1,
                    s if s > target => r-=1,
                    _ => {
                        res.push(quad.clone().into_iter().chain(vec![nums[l] as i32, nums[r] as i32]).collect());
                        l+=1;
                        while l < r && nums[l] == nums[l-1] {l+=1;}
                    }
                }
            }
       } 
    }
    ksum(4, 0, target as i64, &mut nums.into_iter().map(|x| x as i64).collect(), &mut res, &mut quad);
    res
}


pub fn four_sum_new(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    if nums.len() < 4 {return vec![];}    
    nums.sort();
    
    fn ksum(k:usize, start:usize, target:i64, nums:&mut Vec<i32>, mut res:Vec<Vec<i32>>, quad:&mut [i32; 2]) -> Vec<Vec<i32>> {
       if k > 2 {
           for i in start..nums.len()-k+1 {
                if i > start && nums[i] == nums[i-1]{continue;}
                quad[4-k] = nums[i] as i32;
                res = ksum(k-1, i+1, target - nums[i] as i64, nums,res, quad);
           }
       }else {
            let (mut l, mut r):(usize, usize) = (start, nums.len()-1);
            while l < r {
                match nums[l] as i64 + nums[r] as i64 {
                    s if s < target => l+=1,
                    s if s > target => r-=1,
                    _ => {
                        res.push(vec![quad[0], quad[1], nums[l] as i32, nums[r] as i32]);
                        l+=1;
                        while l < r && nums[l] == nums[l-1] {l+=1;}
                    }
                }
            }
       }
       res
    }
    ksum(4, 0, target as i64, &mut nums, vec![], &mut [0, 0])
}

pub fn check(nums: Vec<i32>) -> bool {
        let mut count = 0;
        for (i, v) in nums.iter().enumerate() {
            if v > &nums[(i+1) % nums.len()]{count += 1;}
            if count > 1{return false;}
        }
        true
}

pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let (mut count, mut max):(i32, i32) = (0, 0);
        for (_, &value) in nums.iter().enumerate() {
            if value == 1 {
                count += 1;
                if count > max {max = count;}
            }else {count = 0;}
        }
        max
}

pub fn rotate(nums: &mut Vec<i32>, mut k: i32) {
    fn reverse(nums: &mut Vec<i32>, mut l:i32, mut r:i32){
        while l < r {
            let temp = nums[l as usize];
            nums[l as usize] = nums[r as usize];
            nums[r as usize] = temp;
            l += 1;
            r -= 1;
          }
    }
    k = k % (nums.len() as i32);
    reverse(nums, 0, (nums.len() -1) as i32);
    reverse(nums, 0, k-1);
    reverse(nums, k, (nums.len() -1) as i32);
}
