impl Solution {
    pub fn add_strings(mut num1: String, mut num2: String) -> String {
        let mut n1 = num1.into_bytes();
        let mut n2 = num2.into_bytes();
        n1.reverse();
        n2.reverse();
        let n = n1.len();
        let m = n2.len();
        let mut digits = vec![];
        
        let mut i = 0;
        let mut j = 0;
        let mut sum = 0;
        let mut carry = false;
        loop{
            if n> i || m>j {
                if n>i{
                    sum += n1[i]-b'0';
                    i+=1;
                }
                if m>j{
                    sum += n2[j]-b'0';
                    j+=1;
                }
                if sum < 10 && carry{
                    carry = false;
                }
                if sum >= 10{
                    carry = true;
                    sum -= 10;
                }
                digits.push(sum+b'0');
                if carry {
                    sum = 1;
                }else{
                    sum = 0;
                }
            }else{
                if carry {
                    digits.push(b'1');
                    carry = false;
                }
                break
            }
        }
        digits.reverse();
        return String::from_utf8(digits).unwrap()        
    }
}
