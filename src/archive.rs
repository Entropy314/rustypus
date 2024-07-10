use crate::core::Problem;
use crate::dominance::{ParetoDominance, EpsilonDominance, AttributeDominance};




pub enum DominanceEnum { 
    ParetoDominance,
    EpsilonDominance, 
    AttributeDominance
}
pub struct Archive { 
    pub dominance: DominanceEnum,
    pub content: Vec<T> // IDK WHAT THIS DOES SEE PLATYPUS CODE

}


