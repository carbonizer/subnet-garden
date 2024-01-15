// Copyright 2023-2024 The Milton Hirsch Institute, B.V.
// SPDX-License-Identifier: Apache-2.0

use crate::tests::*;

mod allocate {
    use super::*;
    use subnet_garden::CidrRecord;
    fn new_allocate_test(bits: &str, name: Option<&str>) -> Test {
        let mut test = new_test();
        test.store();
        test.subg.arg("allocate").arg(bits);
        if let Some(name) = name {
            test.subg.arg(name);
        }
        test
    }

    #[test]
    fn allocate_failure() {
        let mut test = new_allocate_test("8", Some("test"));
        test.pool.allocate(16, None).unwrap();
        test.store();
        test.subg
            .assert()
            .failure()
            .code(exitcode::SOFTWARE)
            .stdout("")
            .stderr("Could not allocate subnet\nNo space available\n");
    }

    #[test]
    fn allocate_with_name() {
        let mut test = new_allocate_test("8", Some("test"));
        test.subg.assert().success().stdout("").stderr("");
        test.load();
        let subnets: Vec<&CidrRecord> = test.pool.records().collect();
        assert_eq!(subnets.len(), 1);
        assert_eq!(subnets[0].name.clone().unwrap(), "test");
        assert_eq!(subnets[0].cidr.to_string(), "10.10.0.0/24");
    }

    #[test]
    fn allocate_without_name() {
        let mut test = new_allocate_test("8", None);
        test.subg.assert().success().stdout("").stderr("");
        test.load();
        let subnets: Vec<&CidrRecord> = test.pool.records().collect();
        assert_eq!(subnets.len(), 1);
        assert_eq!(subnets[0].name, None);
        assert_eq!(subnets[0].cidr.to_string(), "10.10.0.0/24");
    }
}

mod free {
    use super::*;
    fn new_free_test(identifier: &str) -> Test {
        let mut test = new_test();
        test.store();
        test.subg.arg("free").arg(identifier);
        test
    }

    #[test]
    fn name_not_found() {
        let mut test = new_free_test("test");
        test.store();
        test.subg
            .assert()
            .failure()
            .code(exitcode::USAGE)
            .stdout("")
            .stderr(
                "Could not parse arg IDENTIFIER\n\
                couldn\'t parse address in network: invalid IP address syntax\n",
            );
    }

    #[test]
    fn free_failure() {
        let mut test = new_free_test("20.20.0.0/24");
        test.store();
        test.subg
            .assert()
            .failure()
            .code(exitcode::SOFTWARE)
            .stdout("")
            .stderr("Could not free subnet 20.20.0.0/24\n");
    }

    #[test]
    fn free_success_with_name() {
        let mut test = new_free_test("test");
        test.pool.allocate(4, Some("test")).unwrap();
        test.store();
        test.subg.assert().success().stdout("").stderr("");
        test.load();
        assert_eq!(test.pool.find_by_name("test"), None);
    }

    #[test]
    fn free_success_with_cidr() {
        let mut test = new_free_test("10.10.0.0/28");
        test.pool.allocate(4, Some("test")).unwrap();
        test.store();
        test.subg.assert().success().stdout("").stderr("");
        test.load();
        assert_eq!(test.pool.find_by_name("test"), None);
    }
}

mod cidrs {
    use super::*;

    fn new_cidrs_test() -> Test {
        let mut test = new_test();
        test.store();
        test.subg.arg("cidrs");
        test
    }

    #[test]
    fn no_cidrs() {
        let mut test = new_cidrs_test();
        test.subg.assert().success().stdout("").stderr("");
    }

    #[test]
    fn has_cidrs() {
        let mut test = new_cidrs_test();
        test.pool.allocate(4, Some("test1")).unwrap();
        test.pool.allocate(6, Some("test2")).unwrap();
        test.store();
        test.subg
            .assert()
            .success()
            .stdout("10.10.0.0/28\n10.10.0.64/26\n")
            .stderr("");
    }

    #[test]
    fn has_cidrs_long() {
        let mut test = new_cidrs_test();
        test.subg.arg("-l");
        test.pool.allocate(4, Some("test1")).unwrap();
        test.pool.allocate(6, None).unwrap();
        test.pool.allocate(6, Some("test2")).unwrap();
        test.store();
        test.subg
            .assert()
            .success()
            .stdout(
                "total 3\n\
                         10.10.0.0/28    test1\n\
                         10.10.0.64/26   -\n\
                         10.10.0.128/26  test2\n",
            )
            .stderr("");
    }
}

mod names {
    use super::*;

    fn new_names_test() -> Test {
        let mut test = new_test();
        test.store();
        test.subg.arg("names");
        test
    }

    #[test]
    fn no_names() {
        let mut test = new_names_test();
        test.subg.assert().success().stdout("").stderr("");
    }

    #[test]
    fn has_names() {
        let mut test = new_names_test();
        test.pool.allocate(4, Some("test1")).unwrap();
        test.pool.allocate(5, None).unwrap();
        test.pool.allocate(6, Some("test2")).unwrap();
        test.pool.allocate(4, Some("test0")).unwrap();
        test.store();
        test.subg
            .assert()
            .success()
            .stdout("test0\ntest1\ntest2\n")
            .stderr("");
    }

    #[test]
    fn has_names_long() {
        let mut test = new_names_test();
        test.subg.arg("-l");
        test.pool.allocate(4, Some("test1")).unwrap();
        test.pool.allocate(5, None).unwrap();
        test.pool.allocate(6, Some("test2")).unwrap();
        test.pool.allocate(4, Some("test-zero")).unwrap();
        test.store();
        test.subg
            .assert()
            .success()
            .stdout(
                "total 3 of 4\n\
                         test-zero  10.10.0.16/28\n\
                         test1      10.10.0.0/28\n\
                         test2      10.10.0.64/26\n",
            )
            .stderr("");
    }
}

mod claim {
    use super::*;
    use subnet_garden::CidrRecord;
    fn new_claim_test(cidr: &str, name: Option<&str>) -> Test {
        let mut test = new_test();
        test.store();
        test.subg.arg("claim").arg(cidr);
        if let Some(name) = name {
            test.subg.arg(name);
        }
        test
    }

    #[test]
    fn claim_failed() {
        let mut test = new_claim_test("20.20.0.0/24", Some("does-not-exist"));
        test.subg
            .assert()
            .failure()
            .code(exitcode::SOFTWARE)
            .stdout("")
            .stderr(
                "Could not claim subnet\n\
                No space available\n",
            );
    }

    #[test]
    fn unnamed() {
        let mut test = new_claim_test("10.10.0.0/24", None);
        test.subg.assert().success().stdout("").stderr("");
        test.load();
        let subnets: Vec<&CidrRecord> = test.pool.records().collect();
        assert_eq!(subnets.len(), 1);
        assert_eq!(subnets[0].name, None);
        assert_eq!(subnets[0].cidr.to_string(), "10.10.0.0/24");
    }

    #[test]
    fn named() {
        let mut test = new_claim_test("10.10.0.0/24", Some("test"));
        test.subg.assert().success().stdout("").stderr("");
        test.load();
        let subnets: Vec<&CidrRecord> = test.pool.records().collect();
        assert_eq!(subnets.len(), 1);
        assert_eq!(subnets[0].name.clone().unwrap(), "test");
        assert_eq!(subnets[0].cidr.to_string(), "10.10.0.0/24");
    }
}

mod rename {
    use super::*;
    use subnet_garden::CidrRecord;
    fn new_rename_test(identifier: &str, name: Option<&str>) -> Test {
        let mut test = new_test();
        test.store();
        test.subg.arg("rename").arg(identifier);
        if let Some(name) = name {
            test.subg.arg(name);
        }
        test
    }

    #[test]
    fn unknown() {
        let mut test = new_rename_test("bad-cidr", None);
        test.subg
            .assert()
            .failure()
            .code(exitcode::USAGE)
            .stdout("")
            .stderr(
                "Could not parse arg IDENTIFIER\n\
                couldn\'t parse address in network: invalid IP address syntax\n",
            );
    }

    #[test]
    fn rename_failure() {
        let mut test = new_rename_test("10.10.0.0/24", Some("test"));
        test.pool.allocate(4, Some("test")).unwrap();
        test.pool.allocate(4, None).unwrap();
        test.store();
        test.subg
            .assert()
            .failure()
            .code(exitcode::SOFTWARE)
            .stdout("")
            .stderr("Could not rename subnet\nDuplicate name\n");
    }

    #[test]
    fn success_with_name() {
        let mut test = new_rename_test("test", Some("test2"));
        test.pool.allocate(4, Some("test")).unwrap();
        test.store();
        test.subg.assert().success().stdout("").stderr("");
        test.load();
        let subnets: Vec<&CidrRecord> = test.pool.records().collect();
        assert_eq!(subnets.len(), 1);
        assert_eq!(subnets[0].name.clone().unwrap(), "test2");
        assert_eq!(subnets[0].cidr.to_string(), "10.10.0.0/28");
    }

    #[test]
    fn success_with_cidr() {
        let mut test = new_rename_test("10.10.0.0/28", Some("test2"));
        test.pool.allocate(4, Some("test")).unwrap();
        test.store();
        test.subg.assert().success().stdout("").stderr("");
        test.load();
        let subnets: Vec<&CidrRecord> = test.pool.records().collect();
        assert_eq!(subnets.len(), 1);
        assert_eq!(subnets[0].name.clone().unwrap(), "test2");
        assert_eq!(subnets[0].cidr.to_string(), "10.10.0.0/28");
    }
}

mod max_available {
    use super::*;
    fn new_max_available_test() -> Test {
        let mut test = new_test();
        test.store();
        test.subg.arg("max-available");
        test
    }

    #[test]
    fn no_subnets() {
        let mut test = new_max_available_test();
        test.subg.assert().success().stdout("16\n").stderr("");
    }

    #[test]
    fn has_subnets() {
        let mut test = new_max_available_test();
        test.pool.allocate(4, Some("test1")).unwrap();
        test.pool.allocate(6, Some("test2")).unwrap();
        test.store();
        test.subg.assert().success().stdout("15\n").stderr("");

        test.pool.allocate(14, Some("test3")).unwrap();
        test.pool.allocate(15, Some("test4")).unwrap();
        test.store();
        test.subg.assert().success().stdout("13\n").stderr("");
    }
}
