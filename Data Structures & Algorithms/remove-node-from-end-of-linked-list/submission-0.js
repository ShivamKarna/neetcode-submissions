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
     * @param {ListNode} head
     * @param {number} n
     * @return {ListNode}
     */
    removeNthFromEnd(head, n) {
        let current = head;
        let size = 0 ;
        while(current){
            current = current.next;
            size++;
        }
        let indexToFind = size - n;
        let i = 1;
        let temp = head;
        if(indexToFind === 0) return head.next;
        while(i < indexToFind){
            temp = temp.next;
            i++;
        }
        temp.next = temp.next.next;
        return head;
    }
}
