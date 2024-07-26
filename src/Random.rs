use std::borrow::BorrowMut;
use ::std::sync::Mutex;
use rand::Rng;
use ::rand::SeedableRng;
pub static random_generator: Mutex<Option<::rand_chacha::ChaCha8Rng>> = Mutex::new(None);


pub fn get_random_f32() -> f32
{
    match random_generator.lock()
    {
        Ok(mut guard) =>
        {
            let option_rng: &mut Option<::rand_chacha::ChaCha8Rng> = guard.borrow_mut();
            match option_rng
            {
                Some(rng) => return rng.gen::<f32>(),
                None =>
                {
                    let mut rng = ::rand_chacha::ChaCha8Rng::seed_from_u64(19878367467713);
                    let number = rng.gen::<f32>();
                    *option_rng = Some(rng);
                    return number;
                },
            }
        },
        Err(poisioned) =>
        {
            let inner = &mut poisioned.into_inner();
            let option_rng: &mut Option<::rand_chacha::ChaCha8Rng> = inner.borrow_mut();
            match option_rng
            {
                Some(rng) => return rng.gen::<f32>(),
                None =>
                {
                    let mut rng = ::rand_chacha::ChaCha8Rng::seed_from_u64(19878367467713);
                    let number = rng.gen::<f32>();
                    *option_rng = Some(rng);
                    return number;
                },
            }
        },
    }
}
pub fn get_random_usize() -> u32
{
    match random_generator.lock()
    {
        Ok(mut guard) =>
        {
            let option_rng: &mut Option<::rand_chacha::ChaCha8Rng> = guard.borrow_mut();
            match option_rng
            {
                Some(rng) => return rng.gen::<u32>(),
                None =>
                {
                    let mut rng = ::rand_chacha::ChaCha8Rng::seed_from_u64(19878367467713);
                    let number = rng.gen::<u32>();
                    return number;
                },
            }
        },
        Err(poisioned) =>
        {
            let inner = &mut poisioned.into_inner();
            let option_rng: &mut Option<::rand_chacha::ChaCha8Rng> = inner.borrow_mut();
            match option_rng
            {
                Some(rng) => return rng.gen::<u32>(),
                None =>
                {
                    let mut rng = ::rand_chacha::ChaCha8Rng::seed_from_u64(19878367467713);
                    let number = rng.gen::<u32>();
                    *option_rng = Some(rng);
                    return number;
                },
            }
        },
    }
}