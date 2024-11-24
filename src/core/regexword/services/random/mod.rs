use std::sync::Arc;
use framework_cqrs_lib::cqrs::models::errors::{Error, ResultErr};
use can_random::CanRandom;

pub mod can_random;

pub trait CanGenerateRandomOrder: Send + Sync {
    fn generate_random_selected_order(&self) -> ResultErr<Vec<u32>>;
}

pub struct RandomOrderGeneratorService {
    pub random_generator: Arc<dyn CanRandom>
}

impl CanGenerateRandomOrder for RandomOrderGeneratorService {
    fn generate_random_selected_order(&self) -> ResultErr<Vec<u32>> {
        let possible_table_size: Vec<Vec<u32>> = vec![
            vec![1u32, 2u32, 3u32],
            vec![1u32, 3u32, 2u32],
            vec![2u32, 1u32, 3u32],
            vec![2u32, 3u32, 1u32],
            vec![3u32, 1u32, 2u32],
            vec![3u32, 2u32, 1u32],
        ];

        let selected = self.random_generator.select_random_in_range(0, (possible_table_size.len() - 1) as u32)?;

        possible_table_size
            .get(selected as usize)
            .map(|d| d.clone())
            .ok_or(Error::Simple(format!("out of range for {selected}")))
    }
}