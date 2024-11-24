use framework_cqrs_lib::cqrs::models::errors::ResultErr;
use rand::Rng;
use crate::core::regexword::services::random_order_generator::RandomOrderGeneratorService;

pub struct RandomOrderGeneratorImpl {}

impl RandomOrderGeneratorService for RandomOrderGeneratorImpl {
    fn select_random_in_range(&self, min: u32, max: u32) -> ResultErr<u32> {
        let mut rng = rand::thread_rng();
        Ok(rng.gen_range(min..=max))
    }
}