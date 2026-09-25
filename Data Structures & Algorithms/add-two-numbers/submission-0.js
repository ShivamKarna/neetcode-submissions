/**
 * Definition for singly-linked list.
 * class ListNode {
 *     constructor(val = 0, next = null) {
 *         this.val = val;
 *         this.next = next;
 *     }
 * }
 */

class Solution {
    /**
     * @param {ListNode} l1
     * @param {ListNode} l2
     * @return {ListNode}
     */
    addTwoNumbers(l1, l2) {
        let dummy = new ListNode(0);
        let current = dummy;
        let carry = 0;
        while(l1 || l2 || carry){
            let a = l1? l1.val :0;
            let b = l2? l2.val : 0;
            let total = a + b + carry;
            carry = Math.floor(total/10);
            current.next = new ListNode(total%10);
            current = current.next;

            l1 = l1? l1.next : null;
            l2 = l2? l2.next: null;
        }
        return dummy.next;
    }
}
