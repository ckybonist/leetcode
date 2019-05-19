// Merge two sorted linked lists and return it as a new list. The new list should be made by splicing together the nodes of the first two lists.
//
// Example:
//
// Input: 1->2->4, 1->3->4
// Output: 1->1->2->3->4->4
//
//


/**
 * Definition for singly-linked list.
 * type ListNode struct {
 *     Val int
 *     Next *ListNode
 * }
 */

/**
 * Special case:
 * [1, 3, 5]
 * [4, 6]
 */ 

func mergeTwoLists(l1 *ListNode, l2 *ListNode) *ListNode {
    if l1 == nil && l2 == nil {
        return nil
    }
    if l1 == nil {
        return l2
    }
    if l2 == nil {
        return l1
    }
    
    // Init start node
    var head *ListNode
    var node *ListNode
    
    if l1.Val < l2.Val {
        head = l1
        node = l1
        l1 = l1.Next
    } else {
        head = l2
        node = l2
        l2 = l2.Next
    }
    
    for l1 != nil && l2 != nil {
        if l1.Val < l2.Val {
            node.Next = l1
            l1 = l1.Next
        } else {
            node.Next = l2
            l2 = l2.Next
        }
        node = node.Next
    }
    
    if l1 != nil {
        node.Next = l1
    }
    if l2 != nil {
        node.Next = l2
    }
    
    return head
}

