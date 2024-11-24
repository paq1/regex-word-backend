use framework_cqrs_lib::cqrs::models::errors::ResultErr;

pub trait CanRandom: Send + Sync {
    fn select_random_in_range(&self, min: u32, max: u32) -> ResultErr<u32>;
}