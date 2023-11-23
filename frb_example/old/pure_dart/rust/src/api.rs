#![allow(unused_variables)]

use std::fmt::Debug;
use std::ops::Deref;
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::Arc;
pub use std::sync::{Mutex, RwLock};
use std::thread::sleep;
use std::time::Duration;

use anyhow::{anyhow, Result};

use backtrace::Backtrace;
use flutter_rust_bridge::*;
use lazy_static::lazy_static;

use crate::data::{EnumAlias, Id, MyEnum, MyStruct, StructAlias, UserIdAlias};
pub use crate::data::{
    FrbOpaqueReturn, FrbOpaqueSyncReturn, HideData, NonCloneData, NonSendHideData,
};
use crate::new_module_system::{use_new_module_system, NewSimpleStruct};
use crate::old_module_system::{use_old_module_system, OldSimpleStruct};
use log::info;

#[cfg(target_family = "wasm")]
mod helpers;

/// Some initialization code to run when the library is first loaded.
#[cfg(not(target_family = "wasm"))]
#[static_init::constructor]
extern "C" fn on_dylib_start() {
    _ = env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format_timestamp(None)
        .try_init();
}

// TODO after enabling zero-copy by default, this is not needed anymore
// pub struct ZeroCopyVecOfPrimitivePack {
//     pub int8list: ZeroCopyBuffer<Vec<i8>>,
//     pub uint8list: ZeroCopyBuffer<Vec<u8>>,
//     pub int16list: ZeroCopyBuffer<Vec<i16>>,
//     pub uint16list: ZeroCopyBuffer<Vec<u16>>,
//     pub uint32list: ZeroCopyBuffer<Vec<u32>>,
//     pub int32list: ZeroCopyBuffer<Vec<i32>>,
//     pub uint64list: ZeroCopyBuffer<Vec<u64>>,
//     pub int64list: ZeroCopyBuffer<Vec<i64>>,
//     pub float32list: ZeroCopyBuffer<Vec<f32>>,
//     pub float64list: ZeroCopyBuffer<Vec<f64>>,
// }
//
// pub fn handle_zero_copy_vec_of_primitive(n: i32) -> ZeroCopyVecOfPrimitivePack {
//     ZeroCopyVecOfPrimitivePack {
//         int8list: ZeroCopyBuffer(vec![42i8; n as usize]),
//         uint8list: ZeroCopyBuffer(vec![42u8; n as usize]),
//         int16list: ZeroCopyBuffer(vec![42i16; n as usize]),
//         uint16list: ZeroCopyBuffer(vec![42u16; n as usize]),
//         int32list: ZeroCopyBuffer(vec![42i32; n as usize]),
//         uint32list: ZeroCopyBuffer(vec![42u32; n as usize]),
//         int64list: ZeroCopyBuffer(vec![42i64; n as usize]),
//         uint64list: ZeroCopyBuffer(vec![42u64; n as usize]),
//         float32list: ZeroCopyBuffer(vec![42.0f32; n as usize]),
//         float64list: ZeroCopyBuffer(vec![42.0f64; n as usize]),
//     }
// }

pub fn handle_list_of_struct(mut l: Vec<MySize>) -> Vec<MySize> {
    info!("handle_list_of_struct({:?})", &l);
    let mut ans = l.clone();
    ans.append(&mut l);
    ans
}

pub fn handle_string_list(names: Vec<String>) -> Vec<String> {
    for name in &names {
        info!("Hello, {}", name);
    }
    names
}

#[derive(Debug, Clone)]
pub struct MyTreeNode {
    pub value_i32: i32,
    pub value_vec_u8: Vec<u8>,
    pub value_boolean: bool,
    pub children: Vec<MyTreeNode>,
}

pub fn handle_complex_struct(s: MyTreeNode) -> MyTreeNode {
    info!("handle_complex_struct({:?})", &s);
    let s_cloned = s.clone();
    s
}

#[derive(Debug, Clone)]
pub struct MyNestedStruct {
    pub tree_node: MyTreeNode,
    pub weekday: Weekdays,
}

pub fn handle_nested_struct(s: MyNestedStruct) -> MyNestedStruct {
    println!("handle_nested_struct({s:?})");
    let s_cloned = s.clone();
    s
}

pub struct MyStreamEntry {
    pub hello: String,
}

// TODO #11193
// https://github.com/fzyzcjy/flutter_rust_bridge/issues/398 reports a compile error like this
pub fn handle_stream_of_struct(sink: StreamSink<MyStreamEntry>) {
    // Ok(())
}

pub fn handle_optional_return(left: f64, right: f64) -> Option<f64> {
    if right == 0. {
        None
    } else {
        Some(left / right)
    }
}

#[derive(Default, Debug, Clone)]
pub struct Element {
    pub tag: Option<String>,
    pub text: Option<String>,
    pub attributes: Option<Vec<Attribute>>,
    pub children: Option<Vec<Element>>,
}

#[derive(Debug, Clone)]
pub struct Attribute {
    pub key: String,
    pub value: String,
}

pub fn handle_optional_struct(document: Option<String>) -> Option<Element> {
    document.map(|inner| Element {
        tag: Some("div".to_owned()),
        attributes: Some(vec![Attribute {
            key: "id".to_owned(),
            value: "root".to_owned(),
        }]),
        children: Some(vec![Element {
            tag: Some("p".to_owned()),
            children: Some(vec![Element {
                text: Some(inner),
                ..Default::default()
            }]),
            ..Default::default()
        }]),
        ..Default::default()
    })
}

#[derive(Debug)]
pub struct ExoticOptionals {
    pub int32: Option<i32>,
    pub int64: Option<i64>,
    pub float64: Option<f64>,
    pub boolean: Option<bool>,
    pub zerocopy: Option<ZeroCopyBuffer<Vec<u8>>>,
    pub int8list: Option<Vec<i8>>,
    pub uint8list: Option<Vec<u8>>,
    pub int32list: Option<Vec<i32>>,
    pub float32list: Option<Vec<f32>>,
    pub float64list: Option<Vec<f64>>,
    pub attributes: Option<Vec<Attribute>>,
    pub attributes_nullable: Vec<Option<Attribute>>,
    pub nullable_attributes: Option<Vec<Option<Attribute>>>,
    pub newtypeint: Option<NewTypeInt>,
}

pub fn handle_optional_increment(opt: Option<ExoticOptionals>) -> Option<ExoticOptionals> {
    fn manipulate_list<T>(src: Option<Vec<T>>, push_value: T) -> Option<Vec<T>> {
        let mut list = src.unwrap_or_default();
        list.push(push_value);
        Some(list)
    }

    opt.map(|mut opt| ExoticOptionals {
        int32: Some(opt.int32.unwrap_or(0) + 1),
        int64: Some(opt.int64.unwrap_or(0) + 1),
        float64: Some(opt.float64.unwrap_or(0.) + 1.),
        boolean: Some(!opt.boolean.unwrap_or(false)),
        int8list: manipulate_list(opt.int8list, 0),
        uint8list: manipulate_list(opt.uint8list, 0),
        int32list: manipulate_list(opt.int32list, 0),
        float32list: manipulate_list(opt.float32list, 0.),
        float64list: manipulate_list(opt.float64list, 0.),
        attributes: Some({
            let mut list = opt.attributes.unwrap_or_default();
            list.push(Attribute {
                key: "some-attrib".to_owned(),
                value: "some-value".to_owned(),
            });
            list
        }),
        nullable_attributes: Some({
            let mut list = opt.nullable_attributes.unwrap_or_default();
            list.push(None);
            list
        }),
        newtypeint: Some({
            let mut val = opt.newtypeint.unwrap_or(NewTypeInt(0));
            val.0 += 1;
            val
        }),
        attributes_nullable: {
            opt.attributes_nullable.push(None);
            opt.attributes_nullable
        },
        zerocopy: Some({
            let mut list = opt.zerocopy.unwrap_or_else(|| ZeroCopyBuffer(vec![]));
            list.0.push(0);
            list
        }),
    })
}

pub fn handle_increment_boxed_optional(opt: Option<Box<f64>>) -> f64 {
    match opt {
        Some(e) => *e + 1.,
        None => 42.,
    }
}

pub struct OptVecs {
    pub i32: Vec<Option<i32>>,
    pub enums: Vec<Option<Weekdays>>,
    pub strings: Vec<Option<String>>,
    pub buffers: Vec<Option<Vec<i32>>>,
}

pub fn handle_vec_of_opts(opt: OptVecs) -> OptVecs {
    fn handle<T>(mut opts: Vec<Option<T>>) -> Vec<Option<T>> {
        opts.push(None);
        opts
    }
    OptVecs {
        i32: handle(opt.i32),
        enums: handle(opt.enums),
        strings: handle(opt.strings),
        buffers: handle(opt.buffers),
    }
}

// Option<Box<T>> can't be sent to Dart,
// but instead can be received by Rust.
pub fn handle_option_box_arguments(
    i8box: Option<Box<i8>>,
    u8box: Option<Box<u8>>,
    i32box: Option<Box<i32>>,
    i64box: Option<Box<i64>>,
    f64box: Option<Box<f64>>,
    boolbox: Option<Box<bool>>,
    structbox: Option<Box<ExoticOptionals>>,
) -> String {
    format!(
        "handle_option_box_arguments({:?})",
        (i8box, u8box, i32box, i64box, f64box, boolbox, structbox)
    )
}

#[derive(Debug, Clone, Copy)]
pub enum Weekdays {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

#[frb]
#[derive(Debug)]
pub struct Note {
    #[frb(default = "Weekdays.Sunday")]
    pub day: Box<Weekdays>,
    pub body: String,
}

pub fn print_note(note: Note) -> ZeroCopyBuffer<Vec<u8>> {
    info!("{:#?}", note);
    ZeroCopyBuffer(vec![1, 2, 3])
}

pub fn handle_return_enum(input: String) -> Option<Weekdays> {
    match input.as_str() {
        "Monday" => Some(Weekdays::Monday),
        "Tuesday" => Some(Weekdays::Tuesday),
        "Wednesday" => Some(Weekdays::Wednesday),
        "Thursday" => Some(Weekdays::Thursday),
        "Friday" => Some(Weekdays::Friday),
        "Saturday" => Some(Weekdays::Saturday),
        "Sunday" => Some(Weekdays::Sunday),
        _ => None,
    }
}

pub fn handle_enum_parameter(weekday: Weekdays) -> Weekdays {
    info!("The weekday is {:?}", weekday);
    weekday
}

#[frb]
#[derive(Debug, Clone)]
pub struct Customized {
    pub final_field: String,
    #[frb(non_final)]
    pub non_final_field: Option<String>,
}

pub fn handle_customized_struct(val: Customized) {
    info!("{:#?}", val);
}

#[frb]
#[derive(Debug)]
pub enum KitchenSink {
    Empty,
    #[frb(unimpl_variant_attr)]
    Primitives {
        #[frb(default = -1)]
        int32: i32,
        #[frb(unimpl_deprecated)]
        float64: f64,
        boolean: bool,
    },
    Nested(
        i32,
        #[frb(default = "KitchenSink.empty()")] Box<KitchenSink>,
    ),
    Optional(#[frb(default = -1)] Option<i32>, Option<i32>),
    Buffer(ZeroCopyBuffer<Vec<u8>>),
    Enums(#[frb(default = "Weekdays.Sunday")] Weekdays),
}

#[frb(unimpl_fn_attr)]
pub fn handle_enum_struct(val: KitchenSink) -> KitchenSink {
    use KitchenSink::*;
    use Weekdays::*;
    let inc = |x| x + 1;
    match val {
        Primitives {
            int32,
            float64,
            boolean,
        } => Primitives {
            int32: int32 + 1,
            float64: float64 + 1.,
            boolean: !boolean,
        },
        Nested(val, nested) => Nested(inc(val), Box::new(handle_enum_struct(*nested))),
        Optional(a, b) => Optional(a.map(inc), b.map(inc)),
        Buffer(ZeroCopyBuffer(mut buf)) => {
            buf.push(1);
            Buffer(ZeroCopyBuffer(buf))
        }
        Enums(day) => Enums(match day {
            Monday => Tuesday,
            Tuesday => Wednesday,
            Wednesday => Thursday,
            Thursday => Friday,
            Friday => Saturday,
            Saturday => Sunday,
            Sunday => Monday,
        }),
        _ => val,
    }
}

// Function that uses imported struct (from within this crate)
pub fn use_imported_struct(my_struct: MyStruct) -> bool {
    my_struct.content
}

// Function that uses imported enum (from within this crate)
pub fn use_imported_enum(my_enum: MyEnum) -> bool {
    match my_enum {
        MyEnum::False => false,
        MyEnum::True => true,
    }
}

// [T; N] example
pub fn get_array() -> [u8; 5] {
    [1, 2, 3, 4, 5]
}

pub struct Point {
    pub x: f32,
    pub y: f32,
}

pub fn get_complex_array() -> [Point; 2] {
    [Point { x: 1.0, y: 1.0 }, Point { x: 2.0, y: 2.0 }]
}

// usize
pub fn get_usize(u: usize) -> usize {
    u
}

/// Example for @freezed and @meta.immutable
#[frb(dart_metadata=("freezed", "immutable" import "package:meta/meta.dart" as meta))]
pub struct UserId {
    #[frb(default = 0)]
    pub value: u32,
}

pub fn next_user_id(user_id: UserId) -> UserId {
    UserId {
        value: user_id.value + 1,
    }
}

// event listener test

lazy_static! {
    static ref EVENTS: Mutex<Option<StreamSink<Event>>> = Default::default();
}

#[frb(dart_metadata = ("freezed"))]
#[derive(Clone)]
pub struct Event {
    pub address: String,
    pub payload: String,
}

impl Event {
    pub fn as_string(&self) -> String {
        format!("{}: {}", self.address, self.payload)
    }
}

pub fn register_event_listener(listener: StreamSink<Event>) -> Result<()> {
    match EVENTS.lock() {
        Ok(mut guard) => {
            *guard = Some(listener);
            Ok(())
        }
        Err(err) => Err(anyhow!("Could not register event listener: {}", err)),
    }
}

pub fn close_event_listener() {
    if let Ok(Some(sink)) = EVENTS.lock().map(|mut guard| guard.take()) {
        sink.close();
    }
}

pub fn create_event(address: String, payload: String) {
    if let Ok(mut guard) = EVENTS.lock() {
        if let Some(sink) = guard.as_mut() {
            sink.add(Event { address, payload });
        }
    }
}

#[derive(Debug, Clone)]
pub struct Log {
    pub key: u32,
    pub value: u32,
}

pub struct SumWith {
    pub x: u32,
}

impl SumWith {
    pub fn sum(&self, y: u32, z: u32) -> u32 {
        self.x + y + z
    }
}

pub fn get_sum_struct() -> SumWith {
    SumWith { x: 21 }
}

pub fn get_sum_array(a: u32, b: u32, c: u32) -> [SumWith; 3] {
    [SumWith { x: a }, SumWith { x: b }, SumWith { x: c }]
}

pub struct ConcatenateWith {
    pub a: String,
}

#[derive(Debug, Clone)]
pub struct Log2 {
    pub key: u32,
    pub value: String,
}

impl ConcatenateWith {
    pub fn new(a: String) -> ConcatenateWith {
        ConcatenateWith { a }
    }
    pub fn concatenate(&self, b: String) -> String {
        format!("{}{b}", self.a)
    }
    pub fn concatenate_static(a: String, b: String) -> String {
        format!("{a}{b}")
    }

    pub fn handle_some_stream_sink(&self, key: u32, max: u32, sink: StreamSink<Log2>) {
        let a = self.a.clone();
        spawn!(|| {
            for i in 0..max {
                sink.add(Log2 {
                    key,
                    value: format!("{a}{i}"),
                });
            }
            sink.close();
        });
    }

    pub fn handle_some_stream_sink_at_1(&self, sink: StreamSink<u32>) {
        spawn!(|| {
            for i in 0..5 {
                sink.add(i);
            }
            sink.close();
        });
    }

    pub fn handle_some_static_stream_sink(key: u32, max: u32, sink: StreamSink<Log2>) {
        spawn!(|| {
            for i in 0..max {
                sink.add(Log2 {
                    key,
                    value: i.to_string(),
                });
            }
            sink.close();
        });
    }

    pub fn handle_some_static_stream_sink_single_arg(sink: StreamSink<u32>) {
        spawn!(|| {
            for i in 0..5 {
                sink.add(i);
            }
            sink.close();
        });
    }
}

#[derive(Debug, Clone)]
#[frb(freezed)]
pub enum Speed {
    Unknown,
    GPS(f64),
}

#[derive(Debug, Clone)]
#[frb(freezed)]
pub enum Distance {
    Unknown,
    Map(f64),
}

#[derive(Debug, Clone)]
#[frb(freezed)]
pub enum Measure {
    Speed(Box<Speed>),
    Distance(Box<Distance>),
}

pub fn multiply_by_ten(measure: Measure) -> Option<Measure> {
    match measure {
        Measure::Speed(b) => match *b {
            Speed::GPS(v) => Some(Measure::Speed(Box::new(Speed::GPS(v * 10.)))),
            Speed::Unknown => None,
        },
        Measure::Distance(b) => match *b {
            Distance::Map(v) => Some(Measure::Distance(Box::new(Distance::Map(v * 10.)))),
            Distance::Unknown => None,
        },
    }
}

pub fn call_old_module_system() -> OldSimpleStruct {
    use_old_module_system(2)
}
pub fn call_new_module_system() -> NewSimpleStruct {
    use_new_module_system(1)
}

pub struct BigBuffers {
    pub int64: Vec<i64>,
    pub uint64: Vec<u64>,
}

pub fn handle_big_buffers() -> BigBuffers {
    BigBuffers {
        int64: vec![i64::MIN, i64::MAX],
        uint64: vec![u64::MAX],
    }
}

pub fn datetime_utc(d: chrono::DateTime<chrono::Utc>) -> chrono::DateTime<chrono::Utc> {
    use chrono::Datelike;
    use chrono::Timelike;
    assert_eq!(&d.year(), &2022);
    assert_eq!(&d.month(), &9);
    assert_eq!(&d.day(), &10);
    assert_eq!(&d.hour(), &20);
    assert_eq!(&d.minute(), &48);
    assert_eq!(&d.second(), &53);
    #[cfg(target_arch = "wasm32")]
    assert_eq!(&d.nanosecond(), &123_000_000);
    #[cfg(not(target_arch = "wasm32"))]
    assert_eq!(&d.nanosecond(), &123_456_000);
    d
}

pub fn datetime_local(d: chrono::DateTime<chrono::Local>) -> chrono::DateTime<chrono::Local> {
    use chrono::Datelike;
    use chrono::Timelike;
    assert_eq!(&d.year(), &2022);
    assert_eq!(&d.month(), &9);
    assert_eq!(&d.day(), &10);
    if cfg!(target_arch = "wasm32") {
        assert_eq!(&d.nanosecond(), &123_000_000);
    } else {
        assert_eq!(&d.hour(), &20);
        assert_eq!(&d.nanosecond(), &123_456_000);
    }
    assert_eq!(&d.minute(), &48);
    assert_eq!(&d.second(), &53);
    d
}

pub fn naivedatetime(d: chrono::NaiveDateTime) -> chrono::NaiveDateTime {
    use chrono::{Datelike, Timelike};
    assert_eq!(&d.year(), &2022);
    assert_eq!(&d.month(), &9);
    assert_eq!(&d.day(), &10);
    assert_eq!(&d.hour(), &20);
    assert_eq!(&d.minute(), &48);
    assert_eq!(&d.second(), &53);
    #[cfg(target_arch = "wasm32")]
    assert_eq!(&d.nanosecond(), &123_000_000);
    #[cfg(not(target_arch = "wasm32"))]
    assert_eq!(&d.nanosecond(), &123_456_000);
    d
}

pub fn optional_empty_datetime_utc(
    d: Option<chrono::DateTime<chrono::Utc>>,
) -> Option<chrono::DateTime<chrono::Utc>> {
    assert_eq!(&d, &None);
    d
}

pub fn duration(d: chrono::Duration) -> chrono::Duration {
    assert_eq!(&d.num_hours(), &4);
    d
}

pub fn handle_timestamps(
    timestamps: Vec<chrono::NaiveDateTime>,
    epoch: chrono::NaiveDateTime,
) -> Vec<chrono::Duration> {
    timestamps
        .into_iter()
        .map(|ts| epoch.signed_duration_since(ts))
        .collect()
}

pub fn handle_durations(
    durations: Vec<chrono::Duration>,
    since: chrono::DateTime<chrono::Local>,
) -> Vec<chrono::DateTime<chrono::Local>> {
    durations.into_iter().map(|dur| since - dur).collect()
}

pub struct TestChrono {
    pub dt: Option<chrono::DateTime<chrono::Utc>>,
    pub dt2: Option<chrono::NaiveDateTime>,
    pub du: Option<chrono::Duration>,
}

pub fn test_chrono() -> TestChrono {
    TestChrono {
        dt: Some(chrono::DateTime::from_utc(
            chrono::NaiveDateTime::from_timestamp_opt(1631297333, 0).unwrap(),
            chrono::Utc,
        )),
        dt2: Some(chrono::NaiveDateTime::from_timestamp_opt(1631297333, 0).unwrap()),
        du: Some(chrono::Duration::hours(4)),
    }
}

pub fn test_precise_chrono() -> TestChrono {
    TestChrono {
        dt: Some(chrono::DateTime::from_utc(
            chrono::NaiveDateTime::from_timestamp_opt(1014466435, 0).unwrap(),
            chrono::Utc,
        )),
        dt2: Some(chrono::NaiveDateTime::from_timestamp_opt(-5362715015, 0).unwrap()),
        du: Some(chrono::Duration::hours(4)),
    }
}

#[derive(Debug, Clone)]
pub struct FeatureChrono {
    pub utc: chrono::DateTime<chrono::Utc>,
    pub local: chrono::DateTime<chrono::Local>,
    pub duration: chrono::Duration,
    pub naive: chrono::NaiveDateTime,
}

pub fn how_long_does_it_take(mine: FeatureChrono) -> anyhow::Result<chrono::Duration> {
    use chrono::{Datelike, Timelike};
    let difference: chrono::Duration = chrono::Utc::now() - mine.utc;
    assert_eq!(&mine.duration.num_hours(), &4);
    assert_eq!(&mine.naive.year(), &2022);
    assert_eq!(&mine.naive.month(), &9);
    assert_eq!(&mine.naive.day(), &10);
    assert_eq!(&mine.naive.hour(), &20);
    assert_eq!(&mine.naive.minute(), &48);
    assert_eq!(&mine.naive.second(), &53);
    #[cfg(target_arch = "wasm32")]
    assert_eq!(&mine.naive.nanosecond(), &123_000_000);
    #[cfg(not(target_arch = "wasm32"))]
    assert_eq!(&mine.naive.nanosecond(), &123_456_000);
    Ok(difference)
}

#[derive(Debug, Clone)]
pub struct FeatureUuid {
    pub one: uuid::Uuid,
    pub many: Vec<uuid::Uuid>,
}

pub fn handle_uuid(id: uuid::Uuid) -> anyhow::Result<uuid::Uuid> {
    Ok(id)
}

pub fn handle_uuids(ids: Vec<uuid::Uuid>) -> anyhow::Result<Vec<uuid::Uuid>> {
    Ok(ids)
}

pub fn handle_nested_uuids(ids: FeatureUuid) -> anyhow::Result<FeatureUuid> {
    Ok(ids)
}

pub struct MessageId(pub [u8; 32]);

pub fn new_msgid(id: [u8; 32]) -> MessageId {
    MessageId(id)
}

pub fn use_msgid(id: MessageId) -> [u8; 32] {
    id.0
}
pub struct Blob(pub [u8; 1600]);
pub fn boxed_blob(blob: Box<[u8; 1600]>) -> Blob {
    Blob(*blob)
}

pub fn use_boxed_blob(blob: Box<Blob>) -> [u8; 1600] {
    blob.0
}

pub struct FeedId(pub [u8; 8]);

pub fn return_boxed_feed_id(id: [u8; 8]) -> Box<FeedId> {
    Box::new(FeedId(id))
}

pub fn return_boxed_raw_feed_id(id: FeedId) -> Box<[u8; 8]> {
    Box::new(id.0)
}

pub struct TestId(pub [i32; 2]);

pub fn test_id(id: TestId) -> TestId {
    id
}

pub fn last_number(array: [f64; 16]) -> f64 {
    array[15]
}

pub fn nested_id(id: [TestId; 4]) -> [TestId; 2] {
    match id {
        [first, .., last] => [first, last],
    }
}

pub fn sync_void() -> SyncReturn<()> {
    SyncReturn(())
}

pub fn handle_type_alias_id(input: Id) -> Id {
    input
}

pub fn handle_type_nest_alias_id(input: UserIdAlias) -> Id {
    input
}
pub struct TestModel {
    pub id: Id,
    pub name: String,
    pub alias_enum: EnumAlias,
    pub alias_struct: MyStruct,
}

pub fn handle_type_alias_model(input: Id) -> TestModel {
    TestModel {
        id: input,
        name: "TestModel".to_owned(),
        alias_enum: EnumAlias::False,
        alias_struct: StructAlias { content: true },
    }
}

#[derive(Debug, Clone)]
pub struct Empty {}

pub fn empty_struct(empty: Empty) -> Empty {
    empty
}

pub fn return_dart_dynamic() -> DartAbi {
    vec!["foo".into_dart()].into_dart()
}

pub struct RawStringItemStruct {
    pub r#type: String,
}

pub fn test_raw_string_item_struct() -> RawStringItemStruct {
    RawStringItemStruct {
        r#type: "test".to_owned(),
    }
}

pub struct MoreThanJustOneRawStringStruct {
    pub regular: String,
    pub r#type: String,
    pub r#async: bool,
    pub another: String,
}

pub fn test_more_than_just_one_raw_string_struct() -> MoreThanJustOneRawStringStruct {
    MoreThanJustOneRawStringStruct {
        regular: "regular".to_owned(),
        r#type: "type".to_owned(),
        r#async: true,
        another: "another".to_owned(),
    }
}

#[frb(mirror(RawStringMirrored))]
pub struct _RawStringMirrored {
    pub r#value: String,
}

#[frb(mirror(NestedRawStringMirrored))]
pub struct _NestedRawStringMirrored {
    pub raw: RawStringMirrored,
}

#[frb(mirror(RawStringEnumMirrored))]
pub enum _RawStringEnumMirrored {
    Raw(RawStringMirrored),
    Nested(NestedRawStringMirrored),
    ListOfNested(ListOfNestedRawStringMirrored),
}

#[frb(mirror(ListOfNestedRawStringMirrored))]
pub struct _ListOfRawNestedStringMirrored {
    pub raw: Vec<NestedRawStringMirrored>,
}

pub fn test_raw_string_mirrored() -> RawStringMirrored {
    RawStringMirrored {
        r#value: "test".to_owned(),
    }
}

pub fn test_nested_raw_string_mirrored() -> NestedRawStringMirrored {
    NestedRawStringMirrored {
        raw: RawStringMirrored {
            r#value: "test".to_owned(),
        },
    }
}

pub fn test_raw_string_enum_mirrored(nested: bool) -> RawStringEnumMirrored {
    if nested {
        RawStringEnumMirrored::Nested(NestedRawStringMirrored {
            raw: RawStringMirrored {
                r#value: "test".to_owned(),
            },
        })
    } else {
        RawStringEnumMirrored::Raw(RawStringMirrored {
            r#value: "test".to_owned(),
        })
    }
}

pub fn test_list_of_raw_nested_string_mirrored() -> ListOfNestedRawStringMirrored {
    ListOfNestedRawStringMirrored {
        raw: vec![NestedRawStringMirrored {
            raw: RawStringMirrored {
                r#value: "test".to_owned(),
            },
        }],
    }
}

pub fn test_fallible_of_raw_string_mirrored() -> Result<Vec<RawStringMirrored>> {
    Ok(vec![RawStringMirrored {
        r#value: "test".to_owned(),
    }])
}

// pub fn test_list_of_nested_enums_mirrored() -> Vec<RawStringEnumMirrored> {
//     vec![
//         RawStringEnumMirrored::Nested(NestedRawStringMirrored {
//             raw: RawStringMirrored {
//                 r#value: "test".to_owned(),
//             },
//         }),
//         RawStringEnumMirrored::Raw(RawStringMirrored {
//             r#value: "test".to_owned(),
//         }),
//     ]
// }

//This seems to be a bug in the syn parser (v1), for whoever tries to fix it, after each failed build you need to manually remove all rust generated files (bridge_*)
// pub fn test_raw_string_item_struct_with_raw_string_in_func(r#type: String) -> RawStringItemStruct {
//     RawStringItemStruct { r#type }
// }

pub fn list_of_primitive_enums(weekdays: Vec<Weekdays>) -> Vec<Weekdays> {
    weekdays
}

pub struct A {
    pub a: String,
}

pub struct B {
    pub b: i32,
}

pub struct C {
    pub c: bool,
}

pub enum Abc {
    A(A),
    B(B),
    C(C),
    JustInt(i32),
}

pub fn test_abc_enum(abc: Abc) -> Abc {
    abc
}

pub struct ContainsMirroredSubStruct {
    pub test: RawStringMirrored,
    pub test2: A,
}

pub fn test_contains_mirrored_sub_struct() -> ContainsMirroredSubStruct {
    ContainsMirroredSubStruct {
        test: RawStringMirrored {
            r#value: "test".to_owned(),
        },
        test2: A {
            a: "test".to_owned(),
        },
    }
}

pub struct StructWithEnum {
    pub abc1: Abc,
    pub abc2: Abc,
}

pub fn test_struct_with_enum(se: StructWithEnum) -> StructWithEnum {
    StructWithEnum {
        abc1: se.abc2,
        abc2: se.abc1,
    }
}

pub fn test_tuple(value: Option<(String, i32)>) -> (String, i32) {
    if let Some((name, value)) = value {
        (format!("Hello {name}"), value + 1)
    } else {
        ("John".to_string(), 0)
    }
}

pub fn test_tuple_2(value: Vec<(String, i32)>) {
    drop(value)
}

pub fn sync_return_mirror() -> SyncReturn<ApplicationSettings> {
    SyncReturn(external_lib::get_app_settings())
}

pub struct SomeStruct {
    pub value: u32,
}

impl SomeStruct {
    pub fn new(value: u32) -> SomeStruct {
        SomeStruct { value }
    }
    pub fn static_return_err_custom_error() -> Result<u32, CustomError> {
        Err(CustomError::Error1 {
            e: 3,
            backtrace: Backtrace::new(),
        })
    }

    pub fn static_return_ok_custom_error() -> Result<u32, CustomError> {
        Ok(3)
    }

    pub fn non_static_return_err_custom_error(&self) -> Result<u32, CustomError> {
        Err(CustomError::Error1 {
            e: self.value,
            backtrace: Backtrace::new(),
        })
    }

    pub fn non_static_return_ok_custom_error(&self) -> Result<u32, CustomError> {
        Ok(self.value)
    }
}

// TODO this is about: Exception + Methods
pub struct CustomStruct {
    pub message: String,
}

impl CustomStruct {
    pub fn new(message: String) -> CustomStruct {
        CustomStruct { message }
    }

    pub fn static_return_custom_struct_error() -> Result<(), CustomStructError> {
        Err(CustomStructError {
            message: "error message".to_string(),
        })
    }

    pub fn static_return_custom_struct_ok() -> Result<u32, CustomStructError> {
        Ok(3)
    }

    pub fn nonstatic_return_custom_struct_error(&self) -> Result<(), CustomStructError> {
        Err(CustomStructError {
            message: self.message.clone(),
        })
    }

    pub fn nonstatic_return_custom_struct_ok(&self) -> Result<u32, CustomStructError> {
        Ok(3)
    }
}
