use core::ptr::NonNull;

pub struct LinkedList<T> {
    pub val: Option<T>,
    pub next: Option<NonNull<LinkedList<T>>>
}

impl LinkedList<i32> {
    /// # Creates an empty LinkedList that may hold i32 values
    /// 
    /// # Example
    /// ```
    /// let list = mini_linked_list::LinkedList::new();
    /// ```
    pub fn new() -> LinkedList<i32> {
        LinkedList {
            val: None,
            next: None
        }
    }

    pub fn collect(&self) -> Vec<i32> {
        let mut result = vec![];
        if self.next.is_none() {
            return result;
        }

        let mut node = self.next.unwrap();
        unsafe {
            result.push(node.as_mut().val.unwrap());
            while node.as_mut().next.is_some() {
                node = node.as_mut().next.unwrap();
                result.push(node.as_mut().val.unwrap());
            }
        };

        result
    }

    pub fn pop_left(&mut self) -> Option<i32> {
        if self.next.is_none() {
            return None;
        } else {
            let mut next = self.next.unwrap();

            let only_one: bool = unsafe {
                next.as_mut().next.is_none()
            };

            if only_one {
                let next_box = unsafe {
                    Box::from_raw(next.as_ptr())
                };
                self.next = None;
                next_box.val
            } else {
                let next_of_next = unsafe {
                    next.as_mut().next
                };
                let next_box = unsafe {
                    Box::from_raw(next.as_ptr())
                };
                self.next = next_of_next;
                next_box.val
            }
        }
    }

    pub fn push_left(&mut self, x: i32) {
        let node = Box::new(
            LinkedList {
                next: None,
                val: Some(x)
            }
        );

        if self.next.is_none() {
            // our list is empty
            // take control over the raw pointer
            let ptr: NonNull<LinkedList<i32>> = Box::leak(node).into();
            self.next = Some(ptr);
        } else {
            let mut ptr: NonNull<LinkedList<i32>> = Box::leak(node).into();
            unsafe {
                ptr.as_mut().next = self.next;
            }
            self.next = Some(ptr);
        }
    }

}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_linked_list_push_left() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.push_left(1);
        list.push_left(2);
        list.push_left(3);
        list.push_left(4);
        assert_eq!(list.collect(), vec![4,3,2,1]);
  
        let x = list.pop_left();
        assert_eq!(x.unwrap(), 4);
  
        let x = list.pop_left();
        assert_eq!(x.unwrap(), 3);
  
        let x = list.pop_left();
        assert_eq!(x.unwrap(), 2);
  
        let x = list.pop_left();
        assert_eq!(x.unwrap(), 1);
  
        let x = list.pop_left();
        assert_eq!(x.is_none(), true);
    }
}
