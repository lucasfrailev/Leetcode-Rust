// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        impl ListNode {
            #[inline]
            fn add(&mut self, value:i32){
                let mut selfreference = self;
                loop {
                    match selfreference.next {
                        Some(ref mut nextnode) => { 
                            selfreference = nextnode;
                            },
                        None => {
                            selfreference.next = Some(Box::new(ListNode::new(value)));
                            break;
                            },
                    }
                }
            }

            fn pop(&mut self) -> Option<i32> {
                match self.next {
                        Some(ref nextnode) => { 
                            let value = nextnode.val;
                            *self = *nextnode.clone();
                            Some(value)
                            },
                        None => None,
                }   
            }
        }
        let mut carry = 0;
        let mut list1 = l1?;
        let mut list2 = l2?;
        let mut sum = list1.val+list2.val;
        // println!("{:#?}",list1);
        let mut remainder = sum % 10;
        carry = (sum-remainder)/10;
        let mut l3 = ListNode::new(remainder);
        loop {
            // println!("{:#?}",l3);
            // println!("{:#?}",list1);
            // println!("{:#?}",list2);
            if let Some(val1) = list1.pop(){
                if let Some(val2) = list2.pop(){
                    sum = val1+val2+carry;
                    remainder = sum % 10;
                    carry = (sum-remainder)/10;
                    l3.add(remainder);
                }else{
                sum = val1+carry;
                remainder = sum % 10;
                carry = (sum-remainder)/10;
                l3.add(remainder);
                }
            }else if let Some(val2) = list2.pop(){
                sum = val2+carry;
                remainder = sum % 10;
                carry = (sum-remainder)/10;
                l3.add(remainder);
            }else{
                if carry != 0{
                    l3.add(carry)
                }
                break
            }
        }        
        return Some(Box::new(l3))
    }
}