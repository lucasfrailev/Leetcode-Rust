impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        use std::collections::HashMap;
        let mut value_map: HashMap<char, i32> = HashMap::with_capacity(7);
        
        value_map.insert('I',1);
        value_map.insert('V',5);
        value_map.insert('X',10);
        value_map.insert('L',50);
        value_map.insert('C',100);
        value_map.insert('D',500);
        value_map.insert('M',1000);
        
        let char_vec: Vec<char> = s.chars().collect();
        
        let mut sum = 0;
        
        for (i, c) in char_vec.iter().enumerate(){
            if i<char_vec.len()-1{
                if value_map.get(&c).unwrap()< value_map.get(&char_vec[i+1]).unwrap(){
                    sum -=value_map.get(&c).unwrap();
                }
                else{
                    sum +=value_map.get(&c).unwrap();
                }
            }
            else{
                sum +=value_map.get(&c).unwrap();
            }
        }
        
        return sum
        
    }
}