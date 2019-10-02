pub fn build_proverb(list: &[&str]) -> String {
    // unimplemented!("build a proverb from this list of items: {:?}", list)
    let mut ans = String::new();
    if list.len() == 0 {
        return ans;
    }
    for i in 0..list.len()-1 {
        ans.push_str(format!("For want of a {} the {} was lost.\n", list[i], list[i+1]).as_str());
    }
    ans.push_str(format!("And all for the want of a {}.", list[0]).as_str());
    ans
}
