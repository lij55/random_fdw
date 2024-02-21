use fake::{Dummy, Fake, Faker};
use pgrx::pg_sys::*;
use pgrx::{
    AnyNumeric, Date, IntoDatum, PgBuiltInOids, PgOid, Time, Timestamp, TimestampWithTimeZone,
};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use rand_chacha;
use rand_chacha::ChaCha8Rng;

pub type DataBuilder = dyn Fn(&mut ChaCha8Rng) -> Option<Datum>;

pub fn create_closure(oid: Oid) -> Box<DataBuilder> {
    let min = 10;
    let max = 1000;
    let max_len = 29;
    // Box::new(move |rng: &mut ThreadRng| -> Cell {
    //     Cell::I64(rng.gen_range(min..max))
    // })
    match oid {
        INT2OID => Box::new(move |rng: &mut ChaCha8Rng| -> Option<Datum> {
            rng.gen_range(min as i16..max as i16).into_datum()
        }),
        FLOAT8ARRAYOID => Box::new(move |rng: &mut ChaCha8Rng| -> Option<Datum> {
            let mut values = Vec::new();
            for _i in 0..1024 {
                values.push(rng.gen_range(-1 as f64..1 as f64))
            }
            values.into_datum()
        }),
        _ => Box::new(move |rng: &mut ChaCha8Rng| -> Option<Datum> { None }),
    }
}

pub fn apply_builder<F>(f: F, rng: &mut ChaCha8Rng) -> Option<Datum>
where
    F: Fn(&mut ChaCha8Rng) -> Option<Datum>,
{
    f(rng)
}

pub fn generate_random_data_for_oid(oid: Oid, rng: &mut ChaCha8Rng) -> Option<Datum> {
    let min_int = 10;
    let max_int = 1000;

    let min_text_len = 10;
    let max_text_len = 29;
    let array_len = 1024;
    let float_factor: u32 = 10;

    match oid {
        INT2OID => rng.gen_range(min_int / 2 as i16..max_int).into_datum(),
        INT4OID => rng.gen_range(min_int..max_int).into_datum(),
        INT8OID => rng
            .gen_range(min_int as i64..max_int as i64 * 2)
            .into_datum(),
        FLOAT4ARRAYOID => {
            let mut values = Vec::new();
            for _i in 0..array_len {
                values.push(rng.gen_range(-1.0 * float_factor as f32..1.0 * float_factor as f32))
            }
            values.into_datum()
        }
        FLOAT8ARRAYOID => {
            let mut values = Vec::new();
            for _i in 0..array_len {
                values.push(rng.gen_range(-1.0 * float_factor as f64..1.0 * float_factor as f64))
            }
            values.into_datum()
        }
        BOOLOID => (rng.gen_range(0..=1) != 0).into_datum(),
        //CHAROID => Some(3.into()),
        FLOAT4OID => rng
            .gen_range(-1.0 * float_factor as f32..1.0 * float_factor as f32)
            .into_datum(),
        FLOAT8OID => rng
            .gen_range(-1.0 * float_factor as f64..1.0 * float_factor as f64)
            .into_datum(),

        NUMERICOID => AnyNumeric::try_from(
            rng.gen_range(-1.0 * float_factor as f32..1.0 * float_factor as f32),
        )
        .unwrap_or_default()
        .into_datum(),

        TEXTOID => (min_text_len..max_text_len)
            .fake_with_rng::<std::string::String, ChaCha8Rng>(rng)
            .into_datum(),

        VARCHAROID => (min_text_len..max_text_len)
            .fake_with_rng::<std::string::String, ChaCha8Rng>(rng)
            .into_datum(),

        BPCHAROID => (min_text_len..max_text_len)
            .fake_with_rng::<std::string::String, ChaCha8Rng>(rng)
            .into_datum(),

        DATEOID => unsafe {
            Date::from_pg_epoch_days(rng.gen_range(1 * 360..50 * 360)).into_datum()
        },
        TIMEOID => Time::new(2, 10, 20.0).into_datum(),
        TIMESTAMPOID => None,
        UUIDOID => None,
        _ => None,
    }
}
