use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, LinkedList};
use std::fmt::Debug;

use rudp2p_derive::{DeserializeData, SerializeData};
use serialize_bits::{des::DeserializerData, ser::SerializerData};

#[derive(Debug, PartialEq)]
enum EnumTest {
    Object,
    Person(String),
}

impl SerializerData for EnumTest {
    fn to_data(&self) -> Vec<u8> {
        let mut res = Vec::new();
        match self {
            EnumTest::Object => {
                res.push(1);
            }
            EnumTest::Person(v) => {
                res.push(2);
                res.append(&mut v.to_data());
            }
        };
        res
    }
}

impl DeserializerData for EnumTest {
    fn from_data(data: &Vec<u8>, index: usize) -> (Self, usize)
    where
        Self: Sized,
    {
        let (code, index) = u8::from_data(data, index);
        if code == 1 {
            (Self::Object, index)
        } else {
            let (value, index) = String::from_data(data, index);
            (Self::Person(value), index)
        }
    }
}

#[derive(Debug, SerializeData, DeserializeData, PartialEq)]
pub struct Test {
    uint: u16,
    int: i128,
    name: String,
    list: Vec<String>,
    child: Child1,
    hash_map: HashMap<String, Vec<bool>>,
    tree_map: BTreeMap<i8, i8>,
    hash_set: HashSet<u16>,
    tree_set: BTreeSet<char>,
    other_list: LinkedList<EnumTest>,
}

#[derive(Debug, SerializeData, DeserializeData, PartialEq)]
pub struct Child1 {
    uint: u16,
    name: String,
    children: Vec<Child2>,
}

#[derive(Debug, SerializeData, DeserializeData, PartialEq)]
pub struct Child2 {
    int: i32,
    ctype: EnumTest,
    option: Option<String>,
}

fn jdd() -> Test {
    let mut map = HashMap::new();
    map.insert(String::from("Key1"), vec![true, false]);

    let mut tree_map = BTreeMap::new();
    tree_map.insert(1, 0);
    tree_map.insert(2, 1);
    tree_map.insert(3, 8);

    let mut set = HashSet::new();
    set.insert(35);

    let mut tree_set = BTreeSet::new();
    tree_set.insert('a');
    tree_set.insert('b');
    tree_set.insert('c');

    let mut other_list = LinkedList::new();
    other_list.push_front(EnumTest::Object);
    other_list.push_front(EnumTest::Person(String::from("Unknown")));

    Test {
        uint: 1,
        int: 36654,
        name: String::from("Test1"),
        list: vec![
            String::from("First element"),
            String::from("Second element"),
        ],
        child: Child1 {
            uint: 3,
            name: String::from("Child"),
            children: vec![
                Child2 {
                    int: 83,
                    ctype: EnumTest::Person(String::from("Male")),
                    option: None,
                },
                Child2 {
                    int: 82,
                    ctype: EnumTest::Object,
                    option: Some(String::from("This is the second child")),
                },
            ],
        },
        hash_map: map,
        tree_map,
        hash_set: set,
        tree_set,
        other_list,
    }
}

#[test]
fn test_serialize() {
    let test1 = jdd();
    let ser = test1.to_data();

    let expected: Vec<u8> = vec![
        1, 0, 46, 143, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 84, 101,
        115, 116, 49, 43, 0, 0, 0, 0, 0, 0, 0, 13, 0, 0, 0, 0, 0, 0, 0, 70, 105, 114, 115, 116, 32,
        101, 108, 101, 109, 101, 110, 116, 14, 0, 0, 0, 0, 0, 0, 0, 83, 101, 99, 111, 110, 100, 32,
        101, 108, 101, 109, 101, 110, 116, 3, 0, 5, 0, 0, 0, 0, 0, 0, 0, 67, 104, 105, 108, 100,
        56, 0, 0, 0, 0, 0, 0, 0, 83, 0, 0, 0, 2, 4, 0, 0, 0, 0, 0, 0, 0, 77, 97, 108, 101, 0, 82,
        0, 0, 0, 1, 1, 24, 0, 0, 0, 0, 0, 0, 0, 84, 104, 105, 115, 32, 105, 115, 32, 116, 104, 101,
        32, 115, 101, 99, 111, 110, 100, 32, 99, 104, 105, 108, 100, 22, 0, 0, 0, 0, 0, 0, 0, 4, 0,
        0, 0, 0, 0, 0, 0, 75, 101, 121, 49, 2, 0, 0, 0, 0, 0, 0, 0, 1, 0, 6, 0, 0, 0, 0, 0, 0, 0,
        1, 0, 2, 1, 3, 8, 2, 0, 0, 0, 0, 0, 0, 0, 35, 0, 3, 0, 0, 0, 0, 0, 0, 0, 97, 98, 99, 17, 0,
        0, 0, 0, 0, 0, 0, 2, 7, 0, 0, 0, 0, 0, 0, 0, 85, 110, 107, 110, 111, 119, 110, 1,
    ];

    assert_eq!(expected, ser);
}

#[test]
fn test_deserialize() {
    let expected = jdd();

    let data: Vec<u8> = vec![
        1, 0, 46, 143, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 84, 101,
        115, 116, 49, 43, 0, 0, 0, 0, 0, 0, 0, 13, 0, 0, 0, 0, 0, 0, 0, 70, 105, 114, 115, 116, 32,
        101, 108, 101, 109, 101, 110, 116, 14, 0, 0, 0, 0, 0, 0, 0, 83, 101, 99, 111, 110, 100, 32,
        101, 108, 101, 109, 101, 110, 116, 3, 0, 5, 0, 0, 0, 0, 0, 0, 0, 67, 104, 105, 108, 100,
        56, 0, 0, 0, 0, 0, 0, 0, 83, 0, 0, 0, 2, 4, 0, 0, 0, 0, 0, 0, 0, 77, 97, 108, 101, 0, 82,
        0, 0, 0, 1, 1, 24, 0, 0, 0, 0, 0, 0, 0, 84, 104, 105, 115, 32, 105, 115, 32, 116, 104, 101,
        32, 115, 101, 99, 111, 110, 100, 32, 99, 104, 105, 108, 100, 22, 0, 0, 0, 0, 0, 0, 0, 4, 0,
        0, 0, 0, 0, 0, 0, 75, 101, 121, 49, 2, 0, 0, 0, 0, 0, 0, 0, 1, 0, 6, 0, 0, 0, 0, 0, 0, 0,
        1, 0, 2, 1, 3, 8, 2, 0, 0, 0, 0, 0, 0, 0, 35, 0, 3, 0, 0, 0, 0, 0, 0, 0, 97, 98, 99, 17, 0,
        0, 0, 0, 0, 0, 0, 2, 7, 0, 0, 0, 0, 0, 0, 0, 85, 110, 107, 110, 111, 119, 110, 1,
    ];
    assert_eq!((expected, 251), Test::from_data(&data, 0));
}
