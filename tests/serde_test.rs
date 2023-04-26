use std::fmt::Debug;

// The unused import mean that it has no test and a reminder of me to write them.
// I'm too lazy to do all of them.
#[allow(unused)]
use sb_sbity::{
    asset::{Costume, Sound},
    block::{
        Block, BlockField, BlockInput, BlockInputValue, BlockMutation, BlockMutationEnum,
        BlockNormal, BlockVarListReporterTop, ShadowInputType,
    },
    broadcast::Broadcast,
    comment::Comment,
    list::List,
    monitor::{ListOrValue, Mode, Monitor, NumberName, Parameter, Slider},
    project::{Meta, Project},
    string_hashmap::StringHashMap,
    target::{RotationStyle, Sprite, SpriteOrStage, Stage, VideoState},
    value::{Float, Int, Name, Number, OpCode, Text, Uid, Value},
    variable::Variable,
};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value as Json;

#[allow(unused)]
fn json_str_equal_debug_json<T>(v_json: &str, file_prefix: &str)
where
    T: DeserializeOwned + Serialize + PartialEq + Debug,
{
    use std::io::Write;
    let v = serde_json::from_str::<T>(v_json)
        .map_err(|e| panic!("{e}"))
        .unwrap();
    let v_json_after = serde_json::to_string(&v)
        .map_err(|e| panic!("{e}"))
        .unwrap();
    let v_after = serde_json::from_str::<T>(&v_json_after).unwrap();

    assert_eq!(v, v_after, "Assert type consistency");
    let v_json = serde_json::from_str::<Json>(v_json).unwrap();
    let v_json_after = serde_json::from_str::<Json>(&v_json_after).unwrap();
    let v_json_str = serde_json::to_string_pretty(&v_json).unwrap();
    let v_json_after_str = serde_json::to_string_pretty(&v_json_after).unwrap();
    let mut v_json_file = std::fs::File::options()
        .write(true)
        .truncate(true)
        .create(true)
        .open(format!("{file_prefix}_before.json"))
        .unwrap();
    let mut v_json_after_file = std::fs::File::options()
        .write(true)
        .truncate(true)
        .create(true)
        .open(format!("{file_prefix}_after.json"))
        .unwrap();
    v_json_file.write_all(v_json_str.as_bytes()).unwrap();
    v_json_after_file
        .write_all(v_json_after_str.as_bytes())
        .unwrap();
}

fn json_str_equal<T>(v_json: &str)
where
    T: DeserializeOwned + Serialize + PartialEq + Debug,
{
    let v = serde_json::from_str::<T>(v_json)
        .map_err(|e| panic!("{e}"))
        .unwrap();
    let v_json_after = serde_json::to_string(&v)
        .map_err(|e| panic!("{e}"))
        .unwrap();
    let v_after = serde_json::from_str::<T>(&v_json_after).unwrap();

    assert_eq!(v, v_after, "Assert type consistency");
    assert_eq!(
        serde_json::from_str::<Json>(v_json).unwrap(),
        serde_json::from_str::<Json>(&v_json_after).unwrap(),
        "Assert json consistency"
    );
}

#[allow(unused)]
fn json_equal<T>(v_json: Json)
where
    T: DeserializeOwned + Serialize + PartialEq + Debug,
{
    let v = serde_json::from_value::<T>(v_json.clone()).unwrap();
    let v_json_after = serde_json::to_string(&v).unwrap();
    let v_after = serde_json::from_str::<T>(&v_json_after).unwrap();

    assert_eq!(v, v_after, "Assert type consistency");
    assert_eq!(
        v_json,
        serde_json::from_str::<Json>(&v_json_after).unwrap(),
        "Assert json consistency"
    );
}

macro_rules! test_json {
    () => {};
    ($($type:ty {$($fname:ident => $json:expr),*})*) => {
        $(
            $(
                #[test]
                fn $fname() {
                    super::super::json_str_equal::<$type>($json)
                }
            )*
        )*
    }
}

#[cfg(test)]
mod serde_test_mod;
