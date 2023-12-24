// Copyright 2023 The Milton Hirsch Institute, B.V.
// SPDX-License-Identifier: Apache-2.0

use crate::util;
use crate::Bits;
use cidr::IpCidr;
use cidr_utils::separator;

#[derive(Debug, PartialEq)]
pub(crate) enum State {
    Allocated,
    Free,
    Unavailable,
}

#[derive(PartialEq, Debug)]
pub(crate) struct Subspace {
    pub(crate) cidr: IpCidr,
    pub(crate) name: Option<String>,
    pub(crate) high: Option<Box<Self>>,
    pub(crate) low: Option<Box<Self>>,
    pub(crate) state: State,
}

impl Subspace {
    pub(crate) fn new(cidr: IpCidr) -> Self {
        Subspace {
            cidr,
            name: None,
            high: None,
            low: None,
            state: State::Free,
        }
    }
    pub(crate) fn host_length(self: &Self) -> Bits {
        return util::max_bits(&self.cidr) - self.cidr.network_length();
    }

    pub(crate) fn split(self: &mut Self) {
        self.state = State::Unavailable;
        let new_network_length = self.cidr.network_length() + 1;
        let low_cidr: IpCidr;
        let high_cidr: IpCidr;
        match self.cidr {
            IpCidr::V4(cidr) => {
                let subnets = separator::Ipv4CidrSeparator::sub_networks(&cidr, new_network_length);
                let subnets_vec = subnets.unwrap();
                low_cidr = IpCidr::V4(*subnets_vec.get(0).unwrap());
                high_cidr = IpCidr::V4(*subnets_vec.get(1).unwrap());
            }
            IpCidr::V6(cidr) => {
                let subnets = separator::Ipv6CidrSeparator::sub_networks(&cidr, new_network_length);
                let subnets_vec = subnets.unwrap();
                low_cidr = IpCidr::V6(*subnets_vec.get(0).unwrap());
                high_cidr = IpCidr::V6(*subnets_vec.get(1).unwrap());
            }
        }
        self.low = Some(Box::new(Subspace::new(low_cidr)));
        self.high = Some(Box::new(Subspace::new(high_cidr)));
    }

    pub(crate) fn find_free_space(&mut self, host_length: Bits) -> Option<&mut Self> {
        if host_length > self.host_length() {
            return None;
        }
        if self.state == State::Free {
            if host_length == self.host_length() {
                return Some(self);
            } else {
                self.split();
            }
        }
        if self.state == State::Unavailable {
            let found_low = self.low.as_deref_mut()?.find_free_space(host_length);
            return match found_low {
                Some(_) => found_low,
                None => self.high.as_deref_mut()?.find_free_space(host_length),
            };
        }
        return None;
    }
    pub(crate) fn free(&mut self, cidr: &IpCidr) -> bool {
        if !util::cidr_contains(&self.cidr, cidr) {
            return false;
        }

        match self.state {
            State::Allocated => match self.cidr == *cidr {
                true => {
                    self.state = State::Free;
                    self.name = None;
                    return true;
                }
                false => {
                    return false;
                }
            },
            State::Free => false,
            State::Unavailable => {
                let low = self.low.as_deref_mut().unwrap();
                let high = self.high.as_deref_mut().unwrap();
                let freed = low.free(cidr) || high.free(cidr);
                if freed {
                    if low.state == State::Free && high.state == State::Free {
                        self.low = None;
                        self.high = None;
                        self.state = State::Free;
                    }
                }

                freed
            }
        }
    }

    pub(crate) fn claim(&mut self, cidr: &IpCidr, name: Option<&str>) -> bool {
        if !util::cidr_contains(&self.cidr, cidr) {
            return false;
        }

        match self.state {
            State::Allocated => return false,
            State::Free => {
                if self.cidr == *cidr {
                    self.state = State::Allocated;
                    self.name = match name {
                        Some(name) => Some(name.to_string()),
                        None => None,
                    };
                    return true;
                }
                self.split();
            }
            State::Unavailable => {}
        }

        if self.low.as_deref_mut().unwrap().claim(cidr, name) {
            return true;
        }

        self.high.as_deref_mut().unwrap().claim(cidr, name)
    }

    pub(crate) fn find_record(&mut self, cidr: &IpCidr) -> Option<&mut Self> {
        if !util::cidr_contains(&self.cidr, cidr) {
            return None;
        }
        if self.cidr == *cidr {
            return Some(self);
        }
        let found_low = self.low.as_deref_mut()?.find_record(cidr);
        return match found_low {
            Some(_) => found_low,
            None => self.high.as_deref_mut()?.find_record(cidr),
        };
    }
}
