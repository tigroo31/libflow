use std::collections::hash_map::{Entry, IntoIter};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;

use serde::{Deserialize, Serialize};
//use serde_with::json::JsonString;
use serde_with::serde_as;

use crate::flow_id::FlowId;
use crate::flow_information::FlowInformation;

#[serde_as]
#[derive(Serialize, Debug, Deserialize, Default)]
pub struct Generator {
    // no map into the json protocol, and a key must be a string.

    // we could convert the FlowId to String
    // with a duplication possible
    //#[serde_as(as = "HashMap<JsonString, _>")]

    // we convert to an array of array
    // with a duplication possible
    #[serde_as(as = "Vec<(_, _)>")]
    flow_map: HashMap<FlowId, FlowInformation>,
}

impl Generator {
    /// Provide a generator with an empty flow map for now.
    pub fn new() -> Self {
        Self { ..Default::default() }
    }

    /// Returns the number of elements in the map.
    pub fn len(&self) -> usize {
        self.flow_map.len()
    }

    /// Returns a mutable reference to the value corresponding to the key.
    ///
    /// The key may be any borrowed form of the map's key type, but
    /// [`Hash`] and [`Eq`] on the borrowed form *must* match those for
    /// the key type.
    ///
    #[inline]
    pub fn get_mut(&mut self, k: &FlowId) -> Option<&mut FlowInformation> {
        self.flow_map.get_mut(k)
    }

    /// Returns `true` if the map contains a value for the specified key.
    #[inline]
    pub fn contains_key(&self, k: &FlowId) -> bool {
        self.flow_map.contains_key(k)
    }

    /// Gets the given key's corresponding entry in the map for in-place manipulation.
    #[inline]
    pub fn entry(&mut self, key: FlowId) -> Entry<'_, FlowId, FlowInformation> {
        self.flow_map.entry(key)
    }

    /// used for IntoIterator trait
    pub fn add(&mut self, key: FlowId, value: FlowInformation) {
        self.flow_map.insert(key, value);
    }
}

impl IntoIterator for Generator {
    type Item = (FlowId, FlowInformation);

    type IntoIter = IntoIter<FlowId, FlowInformation>;

    fn into_iter(self) -> Self::IntoIter {
        self.flow_map.into_iter()
    }
}

pub fn read_from_file<P: AsRef<Path>>(path: P) -> Generator {
    // open the file in read-only mode with buffer.
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    // read the JSON contents of the file
    serde_json::from_reader(reader).unwrap()
}

pub fn write_to_file<P: AsRef<Path>>(generator: &Generator, path: P) {
    // open the file with buffer.
    let file = File::create(path).unwrap();
    let writer = BufWriter::new(file);

    // write the JSON contents to the file
    serde_json::to_writer_pretty(writer, generator).unwrap();
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;
    use std::fs::File;
    use std::io::{Read, Write};
    use std::panic;
    use std::time::Duration;

    use crate::flag::Flag;
    use crate::flow_id::FlowId;
    use crate::flow_information::FlowInformation;
    use crate::generator::{read_from_file, write_to_file, Generator};
    use crate::packet::Packet;

    fn remove_whitespace(s: &str) -> String {
        s.split_whitespace().collect()
    }

    fn basic_flow_map() -> &'static str {
        return r#"
{
  "flow_map": [
    [
      {
        "src": "127.0.0.1",
        "src_port": 8001,
        "dst": "192.168.0.1",
        "dst_port": 8002,
        "transport_protocol": 17
      },
      {
        "backward_packet_list": [],
        "forward_packet_list": []
      }
    ]
  ]
}
"#;
    }

    fn complete_flow_map_version_recto() -> &'static str {
        return r#"
{
  "flow_map": [
    [
      {
        "src": "10.216.31.192",
        "src_port": 0,
        "dst": "10.15.194.132",
        "dst_port": 0,
        "transport_protocol": 132
      },
      {
        "sni": "www.google.com",
        "backward_packet_list": [],
        "forward_packet_list": [
          {
            "length": 0,
            "window": 0,
            "timestamp": {
              "secs": 1595324876,
              "nanos": 73920000
            },
            "flag_list": [],
            "network_protocol": 2048,
            "network_header_length": 5,
            "network_payload_length": 0,
            "position": 1
          }
        ]        
      }
    ],
    [
      {
        "src": "2a01:cb06:a02d:8571:4706:7df1:bd62:5169",
        "src_port": 42254,
        "dst": "2a00:1450:4007:810::2004",
        "dst_port": 443,
        "transport_protocol": 6
      },
      {
        "sni": "mtalk.google.com",
        "backward_packet_list": [
          {
            "length": 218,
            "timestamp": {
              "secs": 1595324883,
              "nanos": 969259000
            },
            "flag_list": [
              "ACK"
            ],
            "network_protocol": 34525,
            "position": 182
          },
          {
            "length": 882,
            "window": 3222,
            "timestamp": {
              "secs": 1595324884,
              "nanos": 248056000
            },
            "flag_list": [
              "ACK"
            ],
            "network_protocol": 34525,
            "position": 196
          }
        ],
        "forward_packet_list": [
          {
            "length": 558,
            "window": 28,
            "timestamp": {
              "secs": 1595324883,
              "nanos": 910142000
            },
            "flag_list": [
              "ACK"
            ],
            "network_protocol": 34525,
            "position": 178
          },
          {
            "length": 64,
            "window": 38,
              "timestamp": {
              "secs": 1595324884,
              "nanos": 36342000
            },
            "flag_list": [
              "ACK"
            ],
            "network_protocol": 34525,
            "network_header_length": 5,
            "network_payload_length": 104,
            "position": 189
          },
          {
            "length": 234,
            "window": 22240,
            "timestamp": {
              "secs": 1595324884,
              "nanos": 158344000
            },
            "flag_list": [
              "ACK", "CWR","ECE"
            ],
            "network_protocol": 34525,
            "position": 194
          }
        ]
      }
    ]    
  ]
}
"#;
    }

    fn complete_flow_map_version_verso() -> &'static str {
        return r#"
{
  "flow_map": [    
    [
      {
        "src": "2a01:cb06:a02d:8571:4706:7df1:bd62:5169",
        "src_port": 42254,
        "dst": "2a00:1450:4007:810::2004",
        "dst_port": 443,
        "transport_protocol": 6
      },
      {
        "sni": "mtalk.google.com",
        "backward_packet_list": [
          {
            "length": 218,
            "timestamp": {
              "secs": 1595324883,
              "nanos": 969259000
            },
            "flag_list": [
              "ACK"
            ],
            "network_protocol": 34525,
            "position": 182
          },
          {
            "length": 882,
            "window": 3222,
            "timestamp": {
              "secs": 1595324884,
              "nanos": 248056000
            },
            "flag_list": [
              "ACK"
            ],
            "network_protocol": 34525,
            "position": 196
          }
        ],
        "forward_packet_list": [
          {
            "length": 558,
            "window": 28,
            "timestamp": {
              "secs": 1595324883,
              "nanos": 910142000
            },
            "flag_list": [
              "ACK"
            ],
            "network_protocol": 34525,
            "position": 178
          },
          {
            "length": 64,
            "window": 38,
            "timestamp": {
              "secs": 1595324884,
              "nanos": 36342000
            },
            "flag_list": [
              "ACK"
            ],
            "network_protocol": 34525,
            "network_header_length": 5,
            "network_payload_length": 104,
            "position": 189
          },
          {
            "length": 234,
            "window": 22240,
            "timestamp": {
              "secs": 1595324884,
              "nanos": 158344000
            },
            "flag_list": [
              "ACK", "CWR","ECE"
            ],
            "network_protocol": 34525,
            "position": 194
          }
        ]
      }
    ],
    [
      {
        "src": "10.216.31.192",
        "src_port": 0,
        "dst": "10.15.194.132",
        "dst_port": 0,
        "transport_protocol": 132
      },
      {
        "sni": "www.google.com",
        "backward_packet_list": [],
        "forward_packet_list": [
          {
            "length": 0,
            "window": 0,
            "timestamp": {
              "secs": 1595324876,
              "nanos": 73920000
            },
            "flag_list": [],
            "network_protocol": 2048,
            "network_header_length": 5,
            "network_payload_length": 0,
            "position": 1
          }
        ]        
      }
    ]
  ]
}
"#;
    }

    fn empty_flow_map() -> &'static str {
        return r#"
{"flow_map":[]}
"#;
    }

    fn bad_flow_map_with_packet_list() -> &'static str {
        return r#"
[
    {
      "packet_list": [],
      "backward_packet_list": [],
      "dst": "10.15.194.131",
      "dst_port": 80,
      "sni": "mtalk.google.com",
      "forward_packet_list": [],
      "src": "10.216.31.191",
      "src_port": 8080,
      "transport_protocol": 17
    }
]
"#;
    }

    fn create_flow_map_file(file_path: &str, flow_map: &str) {
        let mut file = File::create(file_path).unwrap();
        file.write_all(flow_map.as_ref()).unwrap();
    }

    fn assert_file_equal_to_flow_map(file_path: &str, flow_map: &str) {
        let mut file = File::open(file_path).unwrap();
        let mut json = String::new();
        file.read_to_string(&mut json).unwrap();
        assert_eq!(remove_whitespace(flow_map), remove_whitespace(json.as_str()))
    }

    fn create_generator_with_complete_flow() -> Generator {
        let mut generator = Generator::new();

        let flow_id_1 = FlowId::new(132, "10.216.31.192", "10.15.194.132", 0, 0);
        let mut flow_information_1 = FlowInformation::new();
        flow_information_1.sni = Some("www.google.com".to_string());
        flow_information_1.forward_packet_list.push(Packet {
            length: 0,
            window: Some(0),
            timestamp: Duration::new(1595324876, 73920000),
            flag_list: BTreeSet::default(),
            network_protocol: 2048,
            network_payload_length: Some(0),
            network_header_length: Some(5),
            position: 1,
        });
        generator.add(flow_id_1, flow_information_1);

        let flow_id_2 = FlowId::new(
            6,
            "2a01:cb06:a02d:8571:4706:7df1:bd62:5169",
            "2a00:1450:4007:810::2004",
            42254,
            443,
        );
        let mut flow_information_2 = FlowInformation::new();
        flow_information_2.sni = Some("mtalk.google.com".to_string());
        let mut flag_list = BTreeSet::new();
        flag_list.insert(Flag::ACK);
        flow_information_2.backward_packet_list.push(Packet {
            length: 218,
            window: None,
            timestamp: Duration::new(1595324883, 969259000),
            // assumed clone, we provide a copy
            flag_list: flag_list.clone(),
            network_protocol: 34525,
            network_payload_length: None,
            network_header_length: None,
            position: 182,
        });
        flow_information_2.backward_packet_list.push(Packet {
            length: 882,
            window: Some(3222),
            timestamp: Duration::new(1595324884, 248056000),
            // assumed clone, we provide a copy
            flag_list: flag_list.clone(),
            network_protocol: 34525,
            network_payload_length: None,
            network_header_length: None,
            position: 196,
        });
        flow_information_2.forward_packet_list.push(Packet {
            length: 558,
            window: Some(28),
            timestamp: Duration::new(1595324883, 910142000),
            // assumed clone, we provide a copy
            flag_list: flag_list.clone(),
            network_protocol: 34525,
            network_payload_length: None,
            network_header_length: None,
            position: 178,
        });
        flow_information_2.forward_packet_list.push(Packet {
            length: 64,
            window: Some(38),
            timestamp: Duration::new(1595324884, 36342000),
            // assumed clone, we provide a copy
            flag_list: flag_list.clone(),
            network_protocol: 34525,
            network_payload_length: Some(104),
            network_header_length: Some(5),
            position: 189,
        });
        flag_list.insert(Flag::CWR);
        flag_list.insert(Flag::ECE);
        flow_information_2.forward_packet_list.push(Packet {
            length: 234,
            window: Some(22240),
            timestamp: Duration::new(1595324884, 158344000),
            // last flag list, we move
            flag_list,
            network_protocol: 34525,
            network_payload_length: None,
            network_header_length: None,
            position: 194,
        });
        generator.add(flow_id_2, flow_information_2);
        generator
    }

    #[test]
    fn it_can_read_a_basic_flow_map() {
        let file = "target/read_basic_flow.json";
        create_flow_map_file(file, basic_flow_map());

        let generator = read_from_file(file);

        assert_eq!(generator.flow_map.len(), 1);
        let flow_information = generator
            .flow_map
            .get(&FlowId::new(17, "127.0.0.1", "192.168.0.1", 8001, 8002))
            .unwrap();
        assert!(flow_information.forward_packet_list.is_empty());
        assert!(flow_information.backward_packet_list.is_empty());
    }

    #[test]
    fn it_can_read_a_complete_flow_map() {
        let file = "target/read_complete_flow.json";
        create_flow_map_file(file, complete_flow_map_version_recto());

        let generator = read_from_file(file);

        assert_eq!(generator.flow_map.len(), 2);

        let mut flow_information = generator
            .flow_map
            .get(&FlowId::new(132, "10.216.31.192", "10.15.194.132", 0, 0))
            .unwrap();
        assert_eq!(flow_information.sni, Some("www.google.com".to_string()));
        assert_eq!(flow_information.backward_packet_list.len(), 0);
        assert_eq!(flow_information.forward_packet_list.len(), 1);
        let packet = flow_information.forward_packet_list.get(0).unwrap();
        assert_eq!(packet.length, 0);
        assert_eq!(packet.window, Some(0));
        assert_eq!(packet.timestamp, Duration::new(1595324876, 73920000));
        assert_eq!(packet.flag_list, BTreeSet::default());
        assert_eq!(packet.network_protocol, 2048);
        assert_eq!(packet.network_header_length, Some(5));
        assert_eq!(packet.position, 1);

        flow_information = generator
            .flow_map
            .get(&FlowId::new(
                6,
                "2a01:cb06:a02d:8571:4706:7df1:bd62:5169",
                "2a00:1450:4007:810::2004",
                42254,
                443,
            ))
            .unwrap();

        assert_eq!(flow_information.sni, Some("mtalk.google.com".to_string()));
        assert_eq!(flow_information.backward_packet_list.len(), 2);
        let mut packet = flow_information.backward_packet_list.get(1).unwrap();
        assert_eq!(packet.length, 882);
        assert_eq!(packet.window, Some(3222));
        assert_eq!(packet.timestamp, Duration::new(1595324884, 248056000));
        let mut flag_list = BTreeSet::new();
        flag_list.insert(Flag::ACK);
        assert_eq!(packet.flag_list, flag_list);
        assert_eq!(packet.network_protocol, 34525);
        assert_eq!(packet.network_header_length, None);
        assert_eq!(packet.position, 196);

        assert_eq!(flow_information.forward_packet_list.len(), 3);
        packet = flow_information.forward_packet_list.get(2).unwrap();
        assert_eq!(packet.length, 234);
        assert_eq!(packet.window, Some(22240));
        assert_eq!(packet.timestamp, Duration::new(1595324884, 158344000));
        flag_list.insert(Flag::CWR);
        flag_list.insert(Flag::ECE);
        assert_eq!(packet.flag_list, flag_list);
        assert_eq!(packet.network_protocol, 34525);
        assert_eq!(packet.network_header_length, None);
        assert_eq!(packet.position, 194);
    }

    #[test]
    fn it_can_write_a_basic_flow_map() {
        let file = "target/write_basic_flow.csv";
        let mut generator = Generator::new();
        let flow_id = FlowId::new(
            17, // UDP
            "127.0.0.1",
            "192.168.0.1",
            8001,
            8002,
        );
        generator.add(flow_id, FlowInformation::default());

        write_to_file(&generator, file);

        assert_file_equal_to_flow_map(file, basic_flow_map());
    }

    #[test]
    fn it_can_write_a_complete_flow_map() {
        let file = "target/write_complete_flow.csv";

        let generator = create_generator_with_complete_flow();

        write_to_file(&generator, file);

        let result = panic::catch_unwind(|| {
            assert_file_equal_to_flow_map(file, complete_flow_map_version_recto());
        });
        // a hashmap isn't ordered, so 2 elements can be serialized
        // as "A then B" (recto) or B then A (verso)
        if result.is_err() {
            assert_file_equal_to_flow_map(file, complete_flow_map_version_verso());
        }
    }

    #[test]
    fn it_can_write_an_empty_flow_map() {
        let file = "target/write_empty_flow.csv";
        let generator = Generator::new();

        write_to_file(&generator, file);

        assert_file_equal_to_flow_map(file, empty_flow_map());
    }

    #[test]
    #[should_panic]
    fn it_should_panic_when_read_a_flow_path_with_packet_list() {
        let file = "target/bad_packet_list_flow.json";
        create_flow_map_file(file, bad_flow_map_with_packet_list());
        let mut _generator = read_from_file(file);
    }

    #[test]
    fn it_can_browse_a_complete_flow_map() {
        let generator = create_generator_with_complete_flow();

        assert_eq!(generator.len(), 2);

        let expected_flow_id_1 = FlowId::new(132, "10.216.31.192", "10.15.194.132", 0, 0);
        let expected_flow_id_2 = FlowId::new(
            6,
            "2a01:cb06:a02d:8571:4706:7df1:bd62:5169",
            "2a00:1450:4007:810::2004",
            42254,
            443,
        );
        assert_eq!(
            generator.into_iter().fold(0, |n, (flow_id, _flow_information)| {
                let result = panic::catch_unwind(|| {
                    assert_eq!(flow_id, expected_flow_id_1);
                });
                if result.is_err() {
                    assert_eq!(flow_id, expected_flow_id_2);
                };
                n + 1
            }),
            2
        );
    }
}
