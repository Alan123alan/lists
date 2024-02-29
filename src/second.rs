// use std::mem;

//mem::replace(&mut option, None) is such a common idiom that Option type implements it as the 'take' method
//match {None=>None, Some(x)=>Some(y)} is such a common idiom that Option type 
//.as_ref().map(|x|{&**x}) with some syntax sugar can be written as .as_deref()
//.as_ref().map(|x|{&mut**x}) with some syntax sugar can be written as .as_deref_mut()
pub struct List<T>{
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T>{
    elem: T,
    next: Link<T>,
}


impl<T> List<T>{
    pub fn new() -> Self{
        List{head: None}
    }

    pub fn push(&mut self, elem: T){//push new node to the list setting the old head as the next node for the new node pushed that becomes the head
        let new_node = Box::new(Node{//create new node
            elem,
            next: self.head.take(),//set head to None to overwrite it later with new node but first set next node to new node to the old head
        });
        self.head = Some(new_node);//set new node as the new head
    }

    pub fn pop(&mut self) -> Option<T>{//we pop the head and mark the next node as the new head
        self.head
        .take()//replaces self.head current value with None and return self.head value previous to the switch with full ownership
        .map(|node| {//maps to when old self.head value is Some(Box<Node>) otherwise returns None
            self.head = node.next;//set the head to the node.next effectively dropping the old value of self.head
            node.elem //returns the elem from old head which was dropped from the list
        })
    }

    pub fn peek(&self) -> Option<&T>{
        self.head.as_ref().map(|node| &node.elem)//as_ref() demotes the Option<T> to Option<&T>
    }

    pub fn peek_mut(&mut self) -> Option<&mut T>{
        self.head.as_mut().map(|node| &mut node.elem)//as_ref() demotes the Option<T> to Option<&T>
    }

    pub fn into_iter(self)->IntoIter<T>{
        IntoIter(self)
    }

    pub fn iter<'a>(&'a self)->Iter<'a, T>{
        Iter{next: self.head.as_ref().map(|node| &**node)}
    }


    pub fn iter_mut(&mut self)->IterMut<'_, T>{
        // IterMut{next: self.head.as_ref().map(|node| &mut**node)}
        IterMut{next: self.head.as_deref_mut()}
    }
}

impl<T> Drop for List<T>{
    fn drop(&mut self) {
        let mut curr_link = self.head.take();//setting self.head to None after saving current self.head value reference in curr_link?
        while let Some(mut boxed_node) = curr_link{//do while curr_link is enum variant Some(Box<Node>)
            curr_link = boxed_node.next.take();//setting boxed_node to None after saving the boxed_node value reference as in curr_link
        }
    }
}

//into_iter returns the type so full ownership?
pub struct IntoIter<T>(List<T>);

impl<T> Iterator for IntoIter<T>{
    type Item = T;
    fn next(&mut self)->Option<Self::Item>{
        self.0.pop()
    } 
}

//iter returns a immutable reference to the type so no ownership at all just read
pub struct Iter<'a, T>{
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_ref().map(|node| &**node);
            &node.elem
        })
    }
}
//iter_mut returns a mutable reference to the type so no ownership but borrow enough to change the contents in memory
pub struct IterMut<'a, T>{
    next: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            // self.next = node.next.as_ref().map(|node| &mut**node);
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}

#[cfg(test)]
mod test {
    use super::List;
    #[test]
    fn iter_mut(){
        let mut list = List::new();
        list.push(3);
        list.push(2);
        list.push(1);
        let mut mut_iter = list.iter_mut();
        assert_eq!(mut_iter.next(), Some(&mut 1));
        assert_eq!(mut_iter.next(), Some(&mut 2));
        assert_eq!(mut_iter.next(), Some(&mut 3));
    }
    #[test]
    fn iter(){
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3); 
        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
    }
    #[test]
    fn into_iter(){
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);
        let mut iterator = list.into_iter();//an iterator should be mutable since everytime you call next it drops an item?
        assert_eq!(iterator.next(), Some(3));
        assert_eq!(iterator.next(), Some(2));
        assert_eq!(iterator.next(), Some(1));
        assert_eq!(iterator.next(), None);
    }
    #[test]
    fn peek(){
        let mut list = List::<u32>::new();
        list.push(1);
        list.push(2);
        list.push(3);
        list.push(4);
        assert_eq!(list.peek_mut(), Some(&mut 4));
        list.pop();
        assert_eq!(list.peek(), Some(&3));
        list.pop();
        assert_eq!(list.peek_mut(), Some(&mut 2));
        //testing mutability of elem being peeked
        list.peek_mut().map(|elem| {*elem = 44});
        assert_eq!(list.peek(), Some(&44));
    }
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

