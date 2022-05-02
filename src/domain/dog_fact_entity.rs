#[derive(Debug, Clone)]
pub struct DogFactEntity {
    pub fact_id: i32,
    pub fact: String,
}

//TODO remove
pub type DogEntities = Vec<DogFactEntity>;

impl DogFactEntity {
    pub fn new(fact_id: i32, fact: String) -> Self {
        DogFactEntity {
            fact_id,
            fact,
        }
    }
}
