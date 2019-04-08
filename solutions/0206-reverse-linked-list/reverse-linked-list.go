// Reverse a singly linked list.
//
// Example:
//
//
// Input: 1->2->3->4->5->NULL
// Output: 5->4->3->2->1->NULL
//
//
// Follow up:
//
// A linked list can be reversed either iteratively or recursively. Could you implement both?
//


/**
 * Definition for singly-linked list.
 * type ListNode struct {
 *     Val int
 *     Next *ListNode
 * }
 */
func reverseList(head *ListNode) *ListNode {
    //return reverseIter(head)
    //return reverseRecursive(head, nil)
    return reverseRecursiveBackward(head)
}

func reverseIter(current *ListNode) *ListNode {
    var prev *ListNode = nil
    var next *ListNode = nil
    
    for current != nil {
        next = current.Next
        current.Next = prev
        
        prev = current
        current = next
    }
    
    return prev
}

func reverseRecursive(current *ListNode, prev *ListNode) *ListNode {
    if current != nil {
        next := current.Next
        current.Next = prev
        
        prev = current
        current = next
        return reverseRecursive(current, prev)
    }
    
    return prev  // current is nil
}

func reverseRecursiveBackward(head *ListNode) *ListNode {
    if (head == nil || head.Next == nil) {
        return head
    }
    
    p := reverseRecursiveBackward(head.Next)
    // 1 -> 2 -> 3 -> 4 -> 5
    // Assume head is point to element 4
    // The two lines below is mean that: 4 -> 5 becomes 5 -> 4 
    head.Next.Next = head
    head.Next = nil
   
    return p
}
