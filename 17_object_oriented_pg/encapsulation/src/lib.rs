// Encapsulation that Hides Implementation Details

#[derive(Debug)]
pub struct AveregedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveregedCollection {

    pub fn new() -> AveregedCollection {
        AveregedCollection {
            list: vec![],
            average: 0 as f64,
        }
    }

    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            },
            None => None
        }
    }
    
    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}


#[cfg(test)]
mod tests {

    use crate::AveregedCollection;

    #[test]
    fn it_works() {

        let mut ac = AveregedCollection::new();
        ac.add(1);
        ac.add(2);
        ac.add(3);
        
        assert_eq!(ac.average(), 2 as f64);
        println!("result = {:?}", ac);
    }
}
