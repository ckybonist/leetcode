// Given a linked list, swap every two adjacent nodes and return its head.
//
// You may not modify the values in the list's nodes, only nodes itself may be changed.
//
//  
//
// Example:
//
//
// Given 1->2->3->4, you should return the list as 2->1->4->3.
//
//


/**
 * Definition for singly-linked list.
 * public class ListNode {
 *     int val;
 *     ListNode next;
 *     ListNode(int x) { val = x; }
 * }
 */
class Solution {
    public ListNode swapPairs(ListNode head) {
        if (head != null && head.next != null) {
            swap(head);
            
            if (head.next.next != null) {
                swapPairs(head.next.next);
            }
        }
        
        return head;
    }
    
    private ListNode swap(ListNode head) {
        if (head.next != null) {
            int temp = head.val;
            head.val = head.next.val;
            head.next.val = temp;
        }
        
        return head;
    }
}
