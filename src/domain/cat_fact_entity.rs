
#[derive(Debug, Clone)]
pub struct CatFactEntity {
    pub fact_txt: String,
    pub fact_length: i32
}

impl CatFactEntity {
    pub fn new(
        fact_txt: String,
        fact_length: i32
    ) -> Self {
        CatFactEntity {
            fact_txt,
            fact_length
        }
    }
}
