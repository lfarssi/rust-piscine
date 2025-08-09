#[derive(Clone, Debug)]
pub struct List<T> {
    pub head: Option<Node<T>>,
}

#[derive(Clone, Debug)]
pub struct Node<T> {
    pub value: T,
    pub next: Option<Box<Node<T>>>,
}

impl<T: std::fmt::Debug> List<T> {
    pub fn new() -> List<T> {
        List{
            head:None
        }
       
    }

    pub fn push(&mut self, value: T) {
        let node=Node{
            value: value,
            next:  self.head.take().map(Box::new)
        };
        self.head= Some(node);
    }
    
    pub fn pop(&mut self) {
        if let Some(head) =  self.head.take(){
           self.head=head.next.map(|e| *e);
        }

    }   

    pub fn len(&self) -> usize {
        let mut  count=0;
        let mut head = self.head.as_ref();
        while let Some(ele)= head{
                count+=1;
                head = ele.next.as_deref();
                
            
        }
        count
    }
}
