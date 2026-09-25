class Solution {
    reorderList(head) {
        // Handle empty list or single node
        if (!head || !head.next) return;

        // Step 1: Find the middle of the list
        let slow = head;
        let fast = head;
        let prev = null;
        while (fast && fast.next) {
            fast = fast.next.next;
            prev = slow;
            slow = slow.next;
        }

        // Step 2: Split the list into two halves
        prev.next = null;
        let firstHalf = head;
        let secondHalf = slow;

        // Step 3: Reverse the second half
        let prevNode = null;
        let current = secondHalf;
        while (current) {
            let nextTemp = current.next;
            current.next = prevNode;
            prevNode = current;
            current = nextTemp;
        }
        secondHalf = prevNode;

        // Step 4: Merge the two halves
        while (firstHalf && secondHalf) {
            let temp1 = firstHalf.next;
            let temp2 = secondHalf.next;

            firstHalf.next = secondHalf;
            if (temp1) {
                secondHalf.next = temp1;
            }

            firstHalf = temp1;
            secondHalf = temp2;
        }
    }
}