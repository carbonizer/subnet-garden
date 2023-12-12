// Copyright 2023 The Milton Hirsch Institute, B.V.
// SPDX-License-Identifier: Apache-2.0

use super::*;
use crate::model::SubnetGarden;
use cidr::{IpCidr, Ipv4Cidr, Ipv6Cidr};
use std::net::{Ipv4Addr, Ipv6Addr};

static TEST_CIDR4: IpCidr = IpCidr::V4(match Ipv4Cidr::new(Ipv4Addr::new(10, 20, 0, 0), 16) {
    Ok(cidr) => cidr,
    _ => panic!("Failed to create test v4 cidr"),
});

static TEST_CIDR6: IpCidr = IpCidr::V6(
    match Ipv6Cidr::new(Ipv6Addr::new(1, 2, 3, 4, 10, 20, 0, 0), 112) {
        Ok(cidr) => cidr,
        _ => panic!("Failed to create test v6 cidr"),
    },
);

fn new_test_space() -> MemorySubnetGarden {
    let mut instance = MemorySubnetGarden::new();
    instance.new_space("test4", TEST_CIDR4).unwrap();
    instance.new_space("test6", TEST_CIDR6).unwrap();
    return instance;
}

mod memory_garden {
    use super::*;

    mod new {
        use super::*;
        #[test]
        fn new_memory_garden() {
            let instance = MemorySubnetGarden::new();
            assert_eq!(instance.space_count(), 0);
        }
    }

    mod new_space {
        use super::*;

        #[test]
        fn duplicate_object() {
            let mut instance = new_test_space();
            let result = instance.new_space("test4", TEST_CIDR4);
            assert_eq!(result.err(), Some(CreateError::DuplicateObject));
        }

        #[test]
        fn success() {
            let instance = new_test_space();
            assert_eq!(instance.space_count(), 2);
        }
    }

    mod remove_space {
        use super::*;

        #[test]
        fn remove_space_no_such_object() {
            let mut instance = new_test_space();
            let result = instance.remove_space("does-not-exist");
            assert_eq!(result.err(), Some(RemoveError::NoSuchObject));
        }

        #[test]
        fn remove_space_success() {
            let mut instance = new_test_space();
            instance.remove_space("test4").unwrap();
            assert_eq!(instance.space_count(), 1);
        }
    }

    mod space_mut {
        use super::*;

        #[test]
        fn space_success() {
            let mut instance = new_test_space();
            let space = instance.space_mut("test4").unwrap();
            assert_eq!(*space.cidr(), TEST_CIDR4);
        }

        #[test]
        fn space_not_found() {
            let mut instance = new_test_space();
            assert!(instance.space_mut("does-not-exist").is_none());
        }
    }

    mod space_names {
        use super::*;

        #[test]
        fn space_names_success() {
            let instance = new_test_space();
            let mut names = instance.space_names();
            names.sort();
            assert_eq!(names.len(), 2);
            assert_eq!(names[0], "test4");
            assert_eq!(names[1], "test6");
        }

        #[test]
        fn spaces_success() {
            let instance = new_test_space();
            let mut spaces = instance.spaces();
            spaces.sort_by(|a, b| a.cidr().cmp(b.cidr()));
            assert_eq!(spaces.len(), 2);
            assert_eq!(*spaces[0].cidr(), TEST_CIDR4);
            assert_eq!(*spaces[1].cidr(), TEST_CIDR6);
        }

        #[test]
        fn entries_success() {
            let instance = new_test_space();
            let mut entries = instance.entries();
            entries.sort_by(|a, b| a.0.cmp(&b.0));
            assert_eq!(entries.len(), 2);
            assert_eq!(entries[0].0, "test4");
            assert_eq!(*entries[0].1.cidr(), TEST_CIDR4);
            assert_eq!(entries[1].0, "test6");
            assert_eq!(*entries[1].1.cidr(), TEST_CIDR6);
        }
    }
}

mod space {
    use super::*;

    mod allocate {
        use super::*;

        #[test]
        fn too_many_bits() {
            let mut instance = new_test_space();
            let space = instance.space_mut("test4").unwrap();
            let result = space.allocate(17, None);
            assert_eq!(result.err(), Some(AllocateError::NoSpaceAvailable));
        }

        #[test]
        fn no_space_available() {
            let mut instance = new_test_space();
            let space = instance.space_mut("test4").unwrap();
            space.allocate(16, None).unwrap();
            let result = space.allocate(16, None);
            assert_eq!(result.err(), Some(AllocateError::NoSpaceAvailable));
        }

        #[test]
        fn allocate_name_already_exists() {
            let mut instance = new_test_space();
            let space = instance.space_mut("test4").unwrap();
            space.allocate(4, Some("a-name")).unwrap();
            let result = space.allocate(4, Some("a-name"));
            assert_eq!(result.err(), Some(AllocateError::DuplicateName));
        }

        #[test]
        fn allocate_cidr_already_exists() {
            let mut instance = new_test_space();
            let space = instance.space_mut("test4").unwrap();
            space.allocate(4, Some("10.20.0.16/28")).unwrap();
            let result = space.allocate(4, None);
            assert_eq!(result.err(), Some(AllocateError::DuplicateName));
        }

        #[test]
        fn allocate_named() {
            let mut instance = new_test_space();
            let space = instance.space_mut("test4").unwrap();
            let result = space.allocate(4, Some("a-name")).unwrap();
            let looked_up = space.find_by_name("a-name").unwrap();
            assert_eq!(looked_up, result);
        }

        #[test]
        fn allocate_success_v4() {
            let mut instance = new_test_space();
            let space = instance.space_mut("test4").unwrap();
            let result = space.allocate(4, None).unwrap();
            assert_eq!(
                result,
                IpCidr::V4(Ipv4Cidr::new(Ipv4Addr::new(10, 20, 0, 0), 28).unwrap())
            );
        }

        #[test]
        fn allocate_success_v6() {
            let mut instance = new_test_space();
            let space = instance.space_mut("test6").unwrap();
            let result = space.allocate(4, None).unwrap();
            assert_eq!(
                result,
                IpCidr::V6(Ipv6Cidr::new(Ipv6Addr::new(1, 2, 3, 4, 10, 20, 0, 0), 124).unwrap())
            );
        }

        #[test]
        fn allocate_multi_sizes() {
            let mut instance = new_test_space();
            let space = instance.space_mut("test4").unwrap();
            let result1 = space.allocate(4, None).unwrap();
            assert_eq!(
                result1,
                IpCidr::V4(Ipv4Cidr::new(Ipv4Addr::new(10, 20, 0, 0), 28).unwrap())
            );
            let result2 = space.allocate(8, None).unwrap();
            assert_eq!(
                result2,
                IpCidr::V4(Ipv4Cidr::new(Ipv4Addr::new(10, 20, 1, 0), 24).unwrap())
            );
        }

        #[test]
        fn max_bits() {
            let mut instance = new_test_space();
            let space = instance.space_mut("test4").unwrap();
            let result = space.allocate(16, None).unwrap();
            assert_eq!(
                result,
                IpCidr::V4(Ipv4Cidr::new(Ipv4Addr::new(10, 20, 0, 0), 16).unwrap())
            );
        }
    }

    mod claim {
        use super::*;

        #[test]
        fn out_of_range() {
            let mut instance = new_test_space();
            let space = instance.space_mut("test4").unwrap();
            let cidr = IpCidr::V4(Ipv4Cidr::new(Ipv4Addr::new(10, 21, 0, 0), 28).unwrap());
            let result = space.claim(&cidr);
            assert_eq!(result, Err(AllocateError::NoSpaceAvailable));
        }

        #[test]
        fn already_claimed() {
            let mut instance = new_test_space();
            let space = instance.space_mut("test4").unwrap();
            let cidr = IpCidr::V4(Ipv4Cidr::new(Ipv4Addr::new(10, 20, 0, 0), 28).unwrap());
            space.claim(&cidr).unwrap();
            let result = space.claim(&cidr);
            assert_eq!(result, Err(AllocateError::NoSpaceAvailable));
        }

        #[test]
        fn alread_named() {
            let mut instance = new_test_space();
            let space = instance.space_mut("test4").unwrap();
            let cidr = IpCidr::V4(Ipv4Cidr::new(Ipv4Addr::new(10, 20, 0, 0), 28).unwrap());
            space
                .allocate(4, Some(format!("{}", cidr).as_str()))
                .unwrap();
            let result = space.claim(&cidr);
            assert_eq!(result, Err(AllocateError::DuplicateName));
        }

        #[test]
        fn already_allocated() {
            let mut instance = new_test_space();
            let space = instance.space_mut("test4").unwrap();
            let cidr = IpCidr::V4(Ipv4Cidr::new(Ipv4Addr::new(10, 20, 0, 0), 28).unwrap());
            space.allocate(16, None).unwrap();
            let result = space.claim(&cidr);
            assert_eq!(result, Err(AllocateError::NoSpaceAvailable));
        }

        #[test]
        fn success() {
            let mut instance = new_test_space();
            let space = instance.space_mut("test4").unwrap();
            let cidr = IpCidr::V4(Ipv4Cidr::new(Ipv4Addr::new(10, 20, 0, 0), 28).unwrap());
            let result = space.claim(&cidr);
            assert_eq!(result, Ok(()));
        }
    }

    mod names {
        use super::*;

        #[test]
        fn success() {
            let mut instance = new_test_space();
            let space = instance.space_mut("test4").unwrap();
            space.allocate(4, Some("a-name")).unwrap();
            space.allocate(4, Some("b-name")).unwrap();
            space.allocate(4, None).unwrap();
            let mut names = space.names();
            names.sort();
            assert_eq!(names.len(), 3);
            assert_eq!(names[0], "10.20.0.32/28");
            assert_eq!(names[1], "a-name");
            assert_eq!(names[2], "b-name");
        }
    }

    mod cidrs {
        use super::*;

        #[test]
        fn no_cidrs() {
            let mut instance = new_test_space();
            let space = instance.space_mut("test4").unwrap();
            let cidrs = space.cidrs();
            assert_eq!(cidrs.len(), 0);
        }

        #[test]
        fn some() {
            let mut instance = new_test_space();
            let space = instance.space_mut("test4").unwrap();
            space.allocate(4, None).unwrap();
            space.allocate(5, None).unwrap();
            space.allocate(5, None).unwrap();
            space.allocate(4, None).unwrap();
            space.allocate(4, None).unwrap();
            space.allocate(4, None).unwrap();
            let cidrs = space.cidrs();
            assert_eq!(cidrs.len(), 6);
            assert_eq!(
                &IpCidr::V4(Ipv4Cidr::new(Ipv4Addr::new(10, 20, 0, 0), 28).unwrap()),
                cidrs[0],
            );
            assert_eq!(
                &IpCidr::V4(Ipv4Cidr::new(Ipv4Addr::new(10, 20, 0, 16), 28).unwrap()),
                cidrs[1],
            );
            assert_eq!(
                &IpCidr::V4(Ipv4Cidr::new(Ipv4Addr::new(10, 20, 0, 32), 27).unwrap()),
                cidrs[2],
            );
            assert_eq!(
                &IpCidr::V4(Ipv4Cidr::new(Ipv4Addr::new(10, 20, 0, 64), 27).unwrap()),
                cidrs[3],
            );
            assert_eq!(
                &IpCidr::V4(Ipv4Cidr::new(Ipv4Addr::new(10, 20, 0, 96), 28).unwrap()),
                cidrs[4],
            );
            assert_eq!(
                &IpCidr::V4(Ipv4Cidr::new(Ipv4Addr::new(10, 20, 0, 112), 28).unwrap()),
                cidrs[5],
            );
        }
    }

    mod entries {
        use super::*;

        #[test]
        fn success() {
            let mut instance = new_test_space();
            let space = instance.space_mut("test4").unwrap();
            space.allocate(4, Some("a-name")).unwrap();
            space.allocate(4, Some("b-name")).unwrap();
            space.allocate(4, None).unwrap();
            let mut entries = space.entries();
            entries.sort();
            assert_eq!(entries.len(), 3);
            assert_eq!(
                entries[0],
                (
                    String::from("10.20.0.32/28"),
                    IpCidr::V4(Ipv4Cidr::new(Ipv4Addr::new(10, 20, 0, 32), 28).unwrap())
                )
            );
            assert_eq!(
                entries[1],
                (
                    String::from("a-name"),
                    IpCidr::V4(Ipv4Cidr::new(Ipv4Addr::new(10, 20, 0, 0), 28).unwrap())
                )
            );
            assert_eq!(
                entries[2],
                (
                    String::from("b-name"),
                    IpCidr::V4(Ipv4Cidr::new(Ipv4Addr::new(10, 20, 0, 16), 28).unwrap())
                )
            );
        }

        mod serialize {
            use super::*;
            use serde_json::to_string;

            #[test]
            fn success() {
                let mut space = crate::memory::MemorySpace::new(TEST_CIDR4);
                space.allocate(4, Some("a-name")).unwrap();
                space.allocate(4, Some("b-name")).unwrap();
                space.allocate(4, None).unwrap();

                let json = to_string(&space).unwrap();
                assert_eq!(
                    json,
                    "{\
                    \"10.20.0.32/28\":\"10.20.0.32/28\",\
                    \"a-name\":\"10.20.0.0/28\",\
                    \"b-name\":\"10.20.0.16/28\"\
                    }"
                );
            }
        }
    }
}
