use std::cmp::Ordering::*;
use super::Range2;

// https://www.rapidtables.com/math/symbols/Set_Symbols.html
/// Set theory inspired operations
pub trait Set: Sized {
    /// Objects that belong to Self and Other
    /// 
    /// | _ + + + _ _
    /// 
    /// | _ _ + + + _
    /// 
    /// | =>
    /// 
    /// | _ _ + + _ _
    /// 
    /// Returns None if there's no intersection
    fn intersection(&self, other: &Self) -> Option<Self>;

    /// Objects that belong to Self or Other
    /// 
    /// | _ + + + _ _
    /// 
    /// | _ _ + + + _
    /// 
    /// | =>
    /// 
    /// | _ + + + + _
    /// 
    /// If Self and Other are connected, the result will be one,
    /// if not, the result will be two
    fn union(&self, other: &Self) -> Vec<Self>;

    /// All the objects that do not belong to range Self
    /// 
    /// | _ _ _ _ _ _
    /// 
    /// | _ _ + + _ _
    /// 
    /// | =>
    /// 
    /// | + + _ _ + +
    /// 
    /// Returns an empty vector if there is no complement
    fn complement(&self, total_length: usize) -> Vec<Self>;

    /// Objects that belong to Self and not to Other
    /// 
    /// | _ + + + _ _
    /// 
    /// | _ _ + + + _
    /// 
    /// | =>
    /// 
    /// | _ + _ _ _ _
    /// 
    /// Returns an empty vector if there is no complement
    fn relative_complement(&self, other: &Self) -> Vec<Self>;

    /// Objects that belong to Self or Other but not to their intersection
    /// 
    /// | _ + + + _ _
    /// 
    /// | _ _ + + + _
    /// 
    /// | =>
    /// 
    /// | _ + _ _ + _
    /// 
    /// Returns an empty vector if there is no symmetric difference
    fn symmetric_difference(&self, other: &Self) -> Vec<Self>;
}

/// Set theory related operations
pub trait SetSimilar: Sized {
    /// Objects that do not belong to Self or Other, but is between them
    /// 
    /// | _ + _ _ _ _
    /// 
    /// | _ _ _ _ + _
    /// 
    /// | =>
    /// 
    /// | _ _ + + _ _
    /// 
    /// Returns None if there's no intersection
    fn apart(&self, other: &Self) -> Option<Self>;

    /// Objects that belong to Self or Other if Self and Other are connected
    /// 
    /// | _ + + + _ _
    /// 
    /// | _ _ + + + _
    /// 
    /// | =>
    /// 
    /// | _ + + + + _
    /// 
    /// If Self and Other are connected, the result will be Some,
    /// if not, the result will be None.
    fn combine(&self, other: &Self) -> Option<Self>;

    /// Self end equals Other's start or vica versa
    fn adjacent(&self, other: &Self) -> bool;
}

impl Set for Range2 {
    fn intersection(&self, other: &Self) -> Option<Self> {
        match (self.index().cmp(&other.index()), self.end_index().cmp(&other.end_index())) {
            
            // . . * * * * * * . . . .
            // . . . . * * * * * * . .
            // or 
            // . . * * * * . . . . . .
            // . . . . . * * * * * . .
            // or
            // . . * * * * . . . . . .
            // . . . . . . * * * * . .
            
            (Less, Less) => match self.end_index().cmp(&other.index_plus_one()) {

                // . . * * * * . . . . . .
                // . . . . . . * * * * . .

                Greater => None,
                
                // . . * * * * . . . . . .
                // . . . . . * * * * * . .
                
                Equal => None,
                
                // . . * * * * * * . . . .
                // . . . . * * * * * * . .

                Less => Some((other.index()..self.end_index()).into()),
            }
                        
            // . . * * * * * * . . . .
            // . . . . * * * * . . . .
            
            (Less, Equal) => Some((other.index()..self.end_index()).into()),
            
            // . . * * * * * * * * . .
            // . . . . * * * * . . . .
            
            (Less, Greater) => Some((other.index()..other.end_index()).into()),
            
            // . . . . * * * * . . . .
            // . . . . * * * * * * * .
            
            (Equal, Less) => Some((self.index()..self.end_index()).into()),
            
            // . . . . * * * * . . . .
            // . . . . * * * * . . . .
            
            (Equal, Equal) => Some((self.index()..self.end_index()).into()),
            
            // . . . . * * * * * * . .
            // . . . . * * * * . . . .
            
            (Equal, Greater) => Some((self.index()..other.end_index()).into()),
            
            // . . . . * * * * . . . .
            // . . * * * * * * * * . .
            
            (Greater, Less) => Some((self.index()..self.end_index()).into()),
            
            // . . . . * * * * . . . .
            // . . * * * * * * . . . .
            
            (Greater, Equal) => Some((self.index()..self.end_index()).into()),
            
            // . . . . * * * * * * . .
            // . . * * * * * * . . . .
            // or 
            // . . . . . * * * * * . .
            // . . * * * * . . . . . .
            // or
            // . . . . . . * * * * . .
            // . . * * * * . . . . . .
            
            (Greater, Greater) => match self.index_plus_one().cmp(&other.end_index()) {

                // . . . . * * * * * * . .
                // . . * * * * * * . . . .

                Less => Some((self.index()..other.end_index()).into()),

                // . . . . . * * * * * . .
                // . . * * * * . . . . . .

                Equal => None,

                // . . . . . . * * * * . .
                // . . * * * * . . . . . .

                Greater => None,
            }
        }
    }

    fn union(&self, other: &Self) -> Vec<Self> {
        let mut vec = Vec::with_capacity(2);
        
        match (self.index().cmp(&other.index()), self.end_index().cmp(&other.end_index())) {
            
            // . . * * * * * * . . . .
            // . . . . * * * * * * . .
            // or 
            // . . * * * * . . . . . .
            // . . . . . * * * * * . .
            // or
            // . . * * * * . . . . . .
            // . . . . . . * * * * . .
            
            (Less, Less) => match self.end_index().cmp(&other.index_plus_one()) {

                // . . * * * * . . . . . .
                // . . . . . . * * * * . .

                Less => {
                    vec.push((self.index()..self.end_index()).into());
                    vec.push((other.index()..other.end_index()).into());
                }
                
                // . . * * * * . . . . . .
                // . . . . . * * * * * . .
                
                Equal => vec.push((self.index()..other.end_index()).into()),
                
                // . . * * * * * * . . . .
                // . . . . * * * * * * . .

                Greater => vec.push((self.index()..other.end_index()).into()),
            }
                        
            // . . * * * * * * . . . .
            // . . . . * * * * . . . .
            
            (Less, Equal) => vec.push((self.index()..other.end_index()).into()),
            
            // . . * * * * * * * * . .
            // . . . . * * * * . . . .
            
            (Less, Greater) => vec.push((self.index()..self.end_index()).into()),
            
            // . . . . * * * * . . . .
            // . . . . * * * * * * * .
            
            (Equal, Less) => vec.push((self.index()..other.end_index()).into()),
            
            // . . . . * * * * . . . .
            // . . . . * * * * . . . .
            
            (Equal, Equal) => vec.push((self.index()..self.end_index()).into()),
            
            // . . . . * * * * * * . .
            // . . . . * * * * . . . .
            
            (Equal, Greater) => vec.push((self.index()..self.end_index()).into()),
            
            // . . . . * * * * . . . .
            // . . * * * * * * * * . .
            
            (Greater, Less) => vec.push((other.index()..other.end_index()).into()),
            
            // . . . . * * * * . . . .
            // . . * * * * * * . . . .
            
            (Greater, Equal) => vec.push((other.index()..self.end_index()).into()),
            
            // . . . . * * * * * * . .
            // . . * * * * * * . . . .
            // or 
            // . . . . . * * * * * . .
            // . . * * * * . . . . . .
            // or
            // . . . . . . * * * * . .
            // . . * * * * . . . . . .
            
            (Greater, Greater) => match self.index_plus_one().cmp(&other.end_index()) {

                // . . . . * * * * * * . .
                // . . * * * * * * . . . .

                Less => vec.push((other.index()..self.end_index()).into()),

                // . . . . . * * * * * . .
                // . . * * * * . . . . . .

                Equal => vec.push((other.index()..self.end_index()).into()),

                // . . . . . . * * * * . .
                // . . * * * * . . . . . .

                Greater => {
                    vec.push((other.index()..other.end_index()).into());
                    vec.push((self.index()..self.end_index()).into());
                }
            }
        }
        vec
    }

    fn complement(&self, total_length: usize) -> Vec<Self> {
        let mut vec = Vec::with_capacity(2);

        match (self.index() == 0, self.end_index() == total_length) {

            // * * * * * * * * *
            // . . . . . . . . . 
            
            (true, true) => (),
            
            // * * * * * * . . .
            // . . . . . . . . . 
            
            (true, false) => vec.push((self.end_index()..total_length).into()),
            
            // . . . * * * * * *
            // . . . . . . . . . 

            (false, true) => vec.push((0..self.index()).into()),
            
            // . . . * * * . . .
            // . . . . . . . . . 

            (false, false) => {
                vec.push((0..self.index()).into());
                vec.push((self.end_index()..total_length).into());
            }
        }
        vec
    }

    fn relative_complement(&self, other: &Self) -> Vec<Self> {
        let mut vec = Vec::new();

        match (self.index().cmp(&other.index()), self.end_index().cmp(&other.end_index())) {
            
            // . . * * * * * * . . . .
            // . . . . * * * * * * . .
            // or 
            // . . * * * * . . . . . .
            // . . . . . * * * * * . .
            // or
            // . . * * * * . . . . . .
            // . . . . . . * * * * . .
            
            (Less, Less) => match self.end_index().cmp(&other.index_plus_one()) {

                // . . * * * * . . . . . .
                // . . . . . . * * * * . .

                Less => vec.push((self.index()..self.end_index()).into()),
                
                // . . * * * * . . . . . .
                // . . . . . * * * * * . .
                
                Equal => vec.push((self.index()..self.end_index()).into()),
                
                // . . * * * * * * . . . .
                // . . . . * * * * * * . .

                Greater => vec.push((self.index()..other.index()).into()),
            }
                        
            // . . * * * * * * . . . .
            // . . . . * * * * . . . .
            
            (Less, Equal) => vec.push((self.index()..other.index()).into()),
            
            // . . * * * * * * * * . .
            // . . . . * * * * . . . .
            
            (Less, Greater) => {
                vec.push((self.index()..other.index()).into());
                vec.push((other.end_index()..self.end_index()).into());
            }
            
            // . . . . * * * * . . . .
            // . . . . * * * * * * * .
            
            (Equal, Less) => (),
            
            // . . . . * * * * . . . .
            // . . . . * * * * . . . .
            
            (Equal, Equal) => (),
            
            // . . . . * * * * * * . .
            // . . . . * * * * . . . .
            
            (Equal, Greater) => vec.push((other.end_index()..self.end_index()).into()),
            
            // . . . . * * * * . . . .
            // . . * * * * * * * * . .
            
            (Greater, Less) => (),
            
            // . . . . * * * * . . . .
            // . . * * * * * * . . . .
            
            (Greater, Equal) => (),
            
            // . . . . * * * * * * . .
            // . . * * * * * * . . . .
            // or 
            // . . . . . * * * * * . .
            // . . * * * * . . . . . .
            // or
            // . . . . . . * * * * . .
            // . . * * * * . . . . . .
            
            (Greater, Greater) => match self.index_plus_one().cmp(&other.end_index()) {

                // . . . . * * * * * * . .
                // . . * * * * * * . . . .

                Less => vec.push((other.end_index()..self.end_index()).into()),

                // . . . . . * * * * * . .
                // . . * * * * . . . . . .

                Equal => vec.push((self.index()..self.end_index()).into()),

                // . . . . . . * * * * . .
                // . . * * * * . . . . . .

                Greater => vec.push((self.index()..self.end_index()).into()),
            }
        }
        vec
    }

    fn symmetric_difference(&self, other: &Self) -> Vec<Self> {
        let mut vec = Vec::new();

        match (self.index().cmp(&other.index()), self.end_index().cmp(&other.end_index())) {
            
            // . . * * * * * * . . . .
            // . . . . * * * * * * . .
            // or 
            // . . * * * * . . . . . .
            // . . . . . * * * * * . .
            // or
            // . . * * * * . . . . . .
            // . . . . . . * * * * . .

            (Less, Less) => match self.end_index().cmp(&other.index()) {

                // . . * * * * . . . . . .
                // . . . . . . * * * * . .

                Less => {
                    vec.push((self.index()..self.end_index()).into());
                    vec.push((other.index()..other.end_index()).into());
                }
                
                // . . * * * * . . . . . .
                // . . . . . * * * * * . .
                
                Equal => {
                    vec.push((self.index()..self.end_index()).into());
                    vec.push((other.index()..other.end_index()).into());
                }
                
                // . . * * * * * * . . . .
                // . . . . * * * * * * . .

                Greater => {
                    vec.push((self.index()..other.index()).into());
                    vec.push((self.end_index()..other.end_index()).into());
                }
            }
            
            // . . * * * * * * . . . .
            // . . . . * * * * . . . .
            
            (Less, Equal) => vec.push((self.index()..other.index()).into()),
            
            // . . * * * * * * * * . .
            // . . . . * * * * . . . .
            
            (Less, Greater) => {
                vec.push((self.index()..other.index()).into());
                vec.push((other.end_index()..self.end_index()).into());
            }
            
            // . . . . * * * * . . . .
            // . . . . * * * * * * * .
            
            (Equal, Less) => vec.push((self.index()..other.index()).into()),
            
            // . . . . * * * * . . . .
            // . . . . * * * * . . . .
            
            (Equal, Equal) => (),
            
            // . . . . * * * * * * . .
            // . . . . * * * * . . . .
            
            (Equal, Greater) => vec.push((other.end_index()..self.end_index()).into()),
            
            // . . . . * * * * . . . .
            // . . * * * * * * * * . .
            
            (Greater, Less) => {
                vec.push((other.index()..self.index()).into());
                vec.push((self.end_index()..other.end_index()).into());
            }
            
            // . . . . * * * * . . . .
            // . . * * * * * * . . . .
            
            (Greater, Equal) => vec.push((other.index()..self.index()).into()),
            
            // . . . . * * * * * * . .
            // . . * * * * * * . . . .
            // or 
            // . . . . . * * * * * . .
            // . . * * * * . . . . . .
            // or
            // . . . . . . * * * * . .
            // . . * * * * . . . . . .
            
            (Greater, Greater) => match self.index_plus_one().cmp(&other.end_index()) {

                // . . . . * * * * * * . .
                // . . * * * * * * . . . .

                Less => {
                    vec.push((other.index()..self.index()).into());
                    vec.push((other.end_index()..self.end_index()).into());
                }

                // . . . . . * * * * * . .
                // . . * * * * . . . . . .

                Equal => {
                    vec.push((other.index()..other.end_index()).into());
                    vec.push((self.index()..self.end_index()).into());
                }

                // . . . . . . * * * * . .
                // . . * * * * . . . . . .

                Greater => {
                    vec.push((other.index()..other.end_index()).into());
                    vec.push((self.index()..self.end_index()).into());
                }
            }
        }
        vec
    }
}

impl SetSimilar for Range2 {
    fn apart(&self, other: &Self) -> Option<Self> {
        if self.index() < other.index() {
            match self.end_index().cmp(&other.index()) {

                // . . * * * * . . . . . .
                // . . . . . . * * * * . .

                Less => Some((self.end_index()..other.index()).into()),

                // . . * * * * . . . . . .
                // . . . . . * * * * * . .

                Equal => None,

                // . . * * * * * * . . . .
                // . . . . * * * * * * . .

                Greater => None,
            }
        } else {
            match self.index_plus_one().cmp(&other.end_index()) {

                // . . . . * * * * * * . .
                // . . * * * * * * . . . .

                Less => None,
                
                // . . . . . * * * * * . .
                // . . * * * * . . . . . .
                
                Equal => None,

                // . . . . . . * * * * . .
                // . . * * * * . . . . . .

                Greater => Some((other.end_index()..self.index()).into()),
            }
        }
    }

    fn combine(&self, other: &Self) -> Option<Self> {
        let mut res = self.union(other);
        if res.len() == 1 {
            println!("combining ranges {:?} {:?} => {:?}", self.range(), other.range(), res.last().unwrap());
        }
        match res.len() == 1 {
            true => res.pop(),
            false => None,
        }
    }

    fn adjacent(&self, other: &Self) -> bool {
        self.end_index() == other.index_plus_one() || other.end_index() == self.index_plus_one()
    }
}