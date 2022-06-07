use mut_static::MutStatic;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct InstanceStruct {
    pub ip: String,
    pub port: usize,
    pub dom: String,
    pub value: usize,
}

lazy_static! {
    static ref NACOS_INSTANCE_LIST: MutStatic<HashMap<String, InstanceStruct>> = MutStatic::new();
}

pub struct NacosService;

impl NacosService {
    pub fn new() -> Self {
        let mut map: HashMap<String, InstanceStruct> = HashMap::new();
        NACOS_INSTANCE_LIST.set(map).unwrap();
        NacosService
    }

    pub fn foo() {
        let instace_map = InstanceStruct {
            ip: "dd".to_string(),
            port: 8080,
            dom: "ss".to_string(),
            value: 10,
        };

        let mut mut_handle = NACOS_INSTANCE_LIST.write().unwrap();
        mut_handle.insert("dd".to_string(), instace_map);
    }

    pub fn getValue() {
        let value = NACOS_INSTANCE_LIST.read().unwrap();
        println!("{:?}", &value.get("dd"));
    }
}
