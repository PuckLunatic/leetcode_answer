impl Solution {
    pub fn remove_duplicates(s: String) -> String {
        let mut stack: Vec<char> = vec![];
        for c in s.chars() {
            if let Some(&top) = stack.last() {
                if top == c {
                    stack.pop();
                } else {
                    stack.push(c);
                }
            } else {
                stack.push(c)
            }
        }
        stack.iter().collect()
    }
}
