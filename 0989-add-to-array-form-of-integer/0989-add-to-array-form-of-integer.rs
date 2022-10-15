impl Solution {
    pub fn add_to_array_form(num: Vec<i32>,mut k: i32) -> Vec<i32> {
        let div = 10;
        let n = num.len();
        let mut cont = true;
        let mut num2 = vec![];
        let mut carry = false;
        let mut sum = 0;
        let mut i = 0;
        let mut q = k;
        while cont{
            let r = q.rem_euclid(div);
            //println!("{:#?}",n-i);
            if i < n {
            sum += r + num[n-i-1];
            i += 1;
            }else{
                sum += r;
            }
            if sum >=10{
                carry = true;
                sum -= 10;
            }else{
                carry = false;
            }
            num2.push(sum);
            //println!("{:#?}",sum);
            if carry{
                sum = 1;
            }else{
                sum = 0;
                carry = false;
            }
            q = q.div_euclid(div);
            if q == 0 && i == n{
                cont = false;
            }
        }
        if carry{
            num2.push(1);
        }
        num2.reverse();
        return num2
    }
}