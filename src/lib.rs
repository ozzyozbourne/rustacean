use std::mem;

pub fn greet() {
    println!("Hey there rustacean!");
}

pub fn permute(
    nums: &mut Vec<i32>,
    mut res: Vec<Vec<i32>>,
    temp: &mut Vec<i32>,
    used: &mut Vec<bool>,
) -> Vec<Vec<i32>> {
    if used.len() == nums.len() {
        res.push(temp.clone())
    } else {
        for i in 0..nums.len() {
            if !used[i] {
                used[i] = true;
                temp.push(nums[i]);
                res = permute(nums, res, temp, used);
                used[i] = false;
                _ = nums.pop();
            }
        }
    }
    res
}

pub fn permute_heap(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    fn swap(nums: &mut Vec<i32>, i: usize, k: usize) {
        let temp = nums[i];
        nums[i] = nums[k];
        nums[k] = temp;
    }
    fn gen(nums: &mut Vec<i32>, mut res: Vec<Vec<i32>>, k: usize) -> Vec<Vec<i32>> {
        if k == 1 {
            res.push(nums.clone())
        } else {
            res = gen(nums, res, k - 1);
            for i in 0..k - 1 {
                if k % 2 == 0 {
                    swap(nums, i, k - 1);
                } else {
                    swap(nums, 0, k - 1);
                }
                res = gen(nums, res, k - 1);
            }
        }
        res
    }
    let k = nums.len();
    gen(&mut nums, Vec::<Vec<i32>>::new(), k)
}
