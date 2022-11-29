
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
            next: self.next.take(),
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
}


fn main() {
    let mut list = SortedList {
        value: Transaction { id: 23, sender: "Pero".to_string(), receiver: "Momo".to_string(), sum: 450 },
        next: None,
    };

    //list.print();
    list.insert(Transaction { id: 435, sender: "Pero".to_string(), receiver: "Mika".to_string(), sum: 43 });
    list.insert(Transaction { id: 5, sender: "Mika".to_string(), receiver: "Momo".to_string(), sum: 3 });
    list.insert(Transaction { id: 54, sender: "Mika".to_string(), receiver: "Pero".to_string(), sum: 49 });
    list.print();
}
