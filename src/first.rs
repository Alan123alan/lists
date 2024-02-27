use std::mem;

pub struct List{
    head: Link,
}

//pub lets the List enum be accessible from outside of this module
enum Link {
    Empty,
    More(Box<Node>)//Box is a pointer type that uniquely owns a heap allocation of type T
    //Box provides the simplest way of heap allocation in Rust, Boxes provide ownership for this allocation
    //and drop their contents when they go out of scope. Boxes ensure they never allocate more than isize::MAX bytes
    //Recursive structures must be boxed, This is because the size of a recursive structure depends on how many times it calls itself, and so we don't know how much memory to allocate until the recursive structure is actually constructed.
}

struct Node{
    elem: u32,
    next: Link
}

//To associate code with a type `impl` block are used
impl List{
    //Self is an alias for the type we are implementing (List)
    pub fn new() -> Self{
        //Refer to enum variants using `::` which is the namespacing operator
        List {head: Link::Empty}
    }
    //Methods are a special case of function in Rust because of the self argument which doesn't have a declared type
    //There are 3 primary forms self can take: self-(value), &mut self-(mutable reference) and &self-(shared reference)
    //A value represents true ownership can move it, destroy it, mutate it or loan it via a reference
    //A mutable reference represents a temporary exclusive access to a value that you don't own
    pub fn push(&mut self, elem: u32){
        let new_node = Box::new(Node{
            elem,
            next: mem::replace(&mut self.head, Link::Empty),
        });
        self.head = Link::More(new_node);
    }
    pub fn pop(&mut self)->Option<u32>{
        match mem::replace(&mut self.head, Link::Empty ){
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

impl Drop for List{
    fn drop(&mut self) {
        let mut curr_link = mem::replace(&mut self.head, Link::Empty);
        while let Link::More(mut boxed_node) = curr_link{//while curr_link matches Link::More() enum
            curr_link = mem::replace(&mut boxed_node.next, Link::Empty);
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;
    #[test]
    fn basics(){
        let mut list = List::new();

        //check popping an empty list returns 'None'
        assert_eq!(list.pop(), None);

        //populating the list
        list.push(1);
        list.push(2);
        list.push(3);
        list.push(4);
        list.push(5);

        //check that popping works correctly
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));


        //check that pushing still works
        list.push(4);
        list.push(3);
        list.push(2);

        //check removal until emptying the list
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
        
        //check that list gets populated correctly
        // assert_eq!(list, )
    }
}

