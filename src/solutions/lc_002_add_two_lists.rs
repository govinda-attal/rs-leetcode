use crate::ListNode;

pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {

    // as we are traversing the list we want the variables to be mutable
    let mut l1 = &l1;
    let mut l2 = &l2;

    // this is a temp head that starts with zero, but we will return head.next 
    let mut head = Some(Box::new(ListNode::new(0)));
    // as we want to update the current node's next we need a mutable reference
    // and as we need to change the cur variable we need same to be mutable
    let mut cur = &mut head;

    // keeping track of carry
    let mut carry = 0;

    while l1.is_some() ||  l2.is_some() || carry != 0 {
        let v1 = get_val(l1);
        let v2 = get_val(l2);
        if let Some(node) = l1 {
            // move to next
            l1 = &node.next;
        }
        if let Some(node) = l2 {
            // move to next
            l2 = &node.next;
        }
        let total = v1 + v2 + carry;
        carry = total / 10;
        // as we need to update the next get cur's as_mut()
        cur.as_mut().unwrap().next = Some(Box::new(ListNode::new(total % 10)));
        cur = &mut cur.as_mut().unwrap().next;
    }
    // finally return head's next
    head.unwrap().next
}

fn get_val(node: &Option<Box<ListNode>>) -> i32 {
    let Some(node) = node else {
        return 0;
    };
    node.val
}

#[cfg(test)]
mod test{
    use super::*;
    
    #[test]
    fn example1() {

        let l1 = ListNode::from_vec(&[2,4,3]);
        let l2 = ListNode::from_vec(&[5,6,4]);

        assert_eq!(add_two_numbers(l1, l2).unwrap().to_vec(), vec![7,0,8])
        
    }
}