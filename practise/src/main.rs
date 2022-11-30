
#[derive(Clone, Debug)]
struct Transaction {
    id: i32,
    sender: String,
    receiver: String,
    sum: i32
}

#[derive(Clone, Debug)]
struct SortedList {
    value: Transaction,
    next: Option<Box<SortedList>>
}

impl SortedList {
    fn insert(&mut self, new: Transaction) {
        match self.next {
            Some(ref mut nextNode) => {
                if self.value.sum <= new.sum {
                    if nextNode.value.sum >= new.sum {
                        let node = SortedList {
                            value: new,
                            next: self.next.take(),
                        } ;
                        self.next = Some(Box::new(node));
                    }
                    else {
                        nextNode.insert(new)
                    }
                }
                else {
                    let oldHead = SortedList {
                        value: self.value.clone(),
                        next: self.next.take()
                    } ;
                    self.value = new;
                    self.next = Some(Box::new(oldHead));
                }
            },
            None => {
                if self.value.sum > new.sum {
                    let oldHead = SortedList {
                        value: self.value.clone(),
                        next: self.next.take()
                    } ;
                    self.value = new;
                    self.next = Some(Box::new(oldHead));
                }
                else {
                    let node = SortedList {
                        value: new,
                        next: None,
                    } ;
                    self.next = Some(Box::new(node));
                }
            }

        }
    }

    fn print(&mut self) {
        let mut newList = SortedList {
            value: self.value.clone(),
            next: self.next.clone(),
        } ;

        let mut temp = Some(Box::new(newList)); 

        loop {            
            match (temp) {
                Some(s) => { 
                    temp = s.next;
                    print!("{:?}, ", s.value);
                },
                None => return
            }
        }
    }

    fn delete(&mut self, id:i32) {  
        match self.next {
            Some(ref mut nextNode) => {
                if self.value.id == id {
                    self.value = nextNode.value.clone();
                    self.next = nextNode.next.take();
                }
                else {
                    match nextNode.next {
                        Some(ref s) => {
                            nextNode.delete(id);
                        },
                        None => {
                            if nextNode.value.id == id {
                                self.next = None;
                            }
                        }
                    }
                } 
            },
            None => { println!("There is no such transaction with id: {id}"); }
        }
    } 

    fn pop(&mut self) {  
        match self.next {
            Some(ref mut nextNode) => {
                self.value = nextNode.value.clone();
                self.next = nextNode.next.clone();
            },
            None => { println!("Prazna lista"); }
        }
    } 

    fn size(&mut self, i:i32) -> i32 {  
        match self.next {
            Some(ref mut nextNode) => {
                nextNode.size(i+1)
            },
            None => { i }
        }
    } 
}


fn main() {
    let mut list = SortedList {
        value: Transaction { id: 23, sender: "Pero".to_string(), receiver: "Momo".to_string(), sum: 450 },
        next: None,
    };

    
    list.insert(Transaction { id: 435, sender: "Pero".to_string(), receiver: "Mika".to_string(), sum: 43 });
    list.insert(Transaction { id: 5, sender: "Mika".to_string(), receiver: "Momo".to_string(), sum: 3 });
    list.insert(Transaction { id: 54, sender: "Mika".to_string(), receiver: "Pero".to_string(), sum: 49 });
    //list.print();
    println!(" ");
    list.delete(23);
    list.print();
    list.pop();
    list.print();
    println!("size {}", list.size(1));
}
