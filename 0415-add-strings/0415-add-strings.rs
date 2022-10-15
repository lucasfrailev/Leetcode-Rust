impl Solution {
    pub fn add_strings(mut num1: String, mut num2: String) -> String {
        match (&num1,&num2){
            (cmp,_) if cmp == "0"  => num2,
            (_ , cmp) if cmp == "0" => num1,
            (_,_) =>{
                match (num1.pop(),num2.pop()){
                    (Some(n1char),Some(n2char))=>{
                         //println!("{:#?},{:#?}",n1char,n2char);
                        match (char::to_digit(n1char,10),char::to_digit(n2char,10)){
                            (Some(n1),Some(n2))=>{
                                //println!("{:#?},{:#?}",n1,n2);
                                    let sum = n1+n2;
                                    if sum>=10{
                                            let remainder= (sum - 10) as i32;
                                            let carry = "1".to_string();
                                            let mut num3 = remainder.to_string();
                                            //println!("{:#?}",num3);
                                            num3 = Solution::add_strings(Solution::add_strings(carry,num1),num2)+&num3;
                                            return num3
                                        }else{
                                            let mut num3 = sum.to_string();
                                            //println!("{:#?}",num3);
                                            num3 = Solution::add_strings(num1,num2) + &num3;
                                            //println!("{:#?}",num3);
                                            return num3
                                }},
                            (_,_)=> return "r".to_string()}},
                        (Some(n1char),_)=> return num1+&(n1char.to_string()),
                        (_,Some(n2char))=> return num2+&(n2char.to_string()),
                        (_,_)=> return std::string::String::new(),};
                        
                    },
                }
            }
        }
