use std::collections::{HashMap, HashSet};

pub type EID = usize;

pub type Attr<'a> = &'a str;

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
pub enum Val<'a> {
    Str(&'a str),
    Int(i64),
    Ref(EID),
}

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq)]
pub struct Datum<'a> {
    pub eid: EID,
    pub attr: Attr<'a>,
    pub val: Val<'a>,
}

impl<'a> Datum<'a> {
    pub fn new(eid: EID, attr: Attr<'a>, val: Val<'a>) -> Self {
        Datum { eid, attr, val }
    }
}

#[derive(Debug)]
pub struct AVEIndex<'a> {
    index: HashMap<Attr<'a>, HashMap<&'a Val<'a>, HashSet<EID>>>,
}

#[derive(Debug)]
pub struct EAVIndex<'a> {
    index: HashMap<EID, HashMap<Attr<'a>, HashSet<&'a Val<'a>>>>,
}

impl<'a> AVEIndex<'a> {
    pub fn new(data: &'a [Datum]) -> AVEIndex<'a> {
        let mut index = HashMap::new();
        for Datum { eid, attr, val } in data {
            index
                .entry(*attr)
                .or_insert(HashMap::new())
                .entry(val)
                .or_insert(HashSet::new())
                .insert(*eid);
        }

        AVEIndex { index }
    }

    pub fn find<'b>(&'a self, attr: Attr, val: &Val<'b>) -> Option<&'a HashSet<EID>>
    where
        'b: 'a,
    {
        self.index.get(attr).and_then(|ve| ve.get(val))
    }

    pub fn find_one(&self, attr: Attr, val: &Val) -> Option<EID> {
        self.find(attr, val)
            .and_then(|eids| eids.iter().next())
            .copied()
    }
}

impl<'a> EAVIndex<'a> {
    pub fn new(data: &'a [Datum]) -> EAVIndex<'a> {
        let mut index = HashMap::new();
        for Datum { eid, attr, val } in data {
            index
                .entry(*eid)
                .or_insert(HashMap::new())
                .entry(*attr)
                .or_insert(HashSet::new())
                .insert(val);
        }

        EAVIndex { index }
    }

    pub fn find(&'a self, eid: EID, attr: Attr) -> Option<&'a HashSet<&'a Val<'a>>> {
        self.index.get(&eid).and_then(|ve| ve.get(attr))
    }

    pub fn find_one(&self, eid: EID, attr: Attr) -> Option<&Val> {
        self.find(eid, attr)
            .and_then(|vals| vals.iter().next())
            .copied()
    }
}

fn main() {
    let data = vec![
        Datum::new(0, "person/name", Val::Str("Rudoplh".into())),
        Datum::new(0, "person/address", Val::Ref(2)),
        Datum::new(1, "person/name", Val::Str("Ruth".into())),
        Datum::new(1, "person/address", Val::Ref(2)),
        Datum::new(2, "address/line1", Val::Str("123 Some Street")),
        Datum::new(2, "address/line2", Val::Str("Some Suburb")),
    ];

    let ave_index = AVEIndex::new(&data);
    let eav_index = EAVIndex::new(&data);

    println!("{:?}", &ave_index);
    println!("{:?}", &eav_index);
    println!("Find Ruth!");

    if let Some(eid) = ave_index.find_one("person/name", &Val::Str("Ruth")) {
        if let Some(Val::Ref(addr_eid)) = eav_index.find_one(eid, "person/address") {
            let name = match eav_index.find_one(eid, "person/name") {
                Some(Val::Str(name)) => name,
                _ => "",
            };
            let line1 = match eav_index.find_one(*addr_eid, "address/line1") {
                Some(Val::Str(line)) => line,
                _ => "",
            };
            let line2 = match eav_index.find_one(*addr_eid as usize, "address/line2") {
                Some(Val::Str(line)) => line,
                _ => "",
            };
            println!("Ruth: {:?}", (eid, name, line1, line2));
        } else {
            println!("Could not find address");
        }
    } else {
        println!("Could not find Ruth");
    }
}
