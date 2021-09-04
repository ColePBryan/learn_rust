use std::{collections::HashMap};

pub struct QueryString<'buf> {
    data: HashMap<&'buf str, Value<'buf>>
}

pub enum Value<'buf>{
    Single(&'buf str),
    Multiple(Vec<&'buf str>),
}

impl<'buf> QueryString<'buf> {
    pub fn get (&self, key: &str) -> Option<&Value>{
        self.data.get(key)
    }
}

impl<'buf> From<&'buf str> for QueryString<'buf>{
    fn from(s: &'buf str) -> Self {
        let mut data = HashMap::new();
        for sub_str in s.split('&'){
            let mut key = sub_str;
            let mut val = "";

            if let Some(split_index) = sub_str.find('=') {
                key = &sub_str[..split_index];
                val = &sub_str[split_index + 1..];
            }
            // data.insert(key, val);
            data.entry(key)
                .and_modify(|existing:&mut Value| match existing {
                    Value::Single(prev_value)=>{
                        *existing = Value::Multiple(vec![prev_value, val]);
                    },
                    Value::Multiple(prev_vector)=>prev_vector.push(val)
                })
                .or_insert(Value::Single(val));
        }
        QueryString{data}
    }
}