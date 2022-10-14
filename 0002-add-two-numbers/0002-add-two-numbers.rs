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
        match (l1, l2) {
            (None, None) => None,
            (Some(n), None) | (None, Some(n)) => Some(n), 
            (Some(list1), Some(list2)) => {
                let sum = list1.val+list2.val;
                if sum >= 10 {
                    let remainder = sum - 10;
                    let list3 = Some(Box::new(ListNode::new(1))); 
                    Some(Box::new(ListNode{
                        val: remainder,
                        next: Solution::add_two_numbers(Solution::add_two_numbers(list3,list1.next),list2.next),
                        }))
                }else {
                    Some(Box::new(ListNode{
                        val: sum,
                        next: Solution::add_two_numbers(list1.next,list2.next),
                        }))
                }
            }
        }
    }
}