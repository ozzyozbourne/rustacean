fn main() {
    let (mut bunnies , mut carrots) = (1,2);
    const FOO_BAR:isize = 10;
    {
        let mut bunnies = 12;
        println!("{}   {}",bunnies, FOO_BAR );
    }
     println!("{}    {}",bunnies, FOO_BAR );
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

