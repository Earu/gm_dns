#![feature(c_unwind)]

use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::net::Ipv6Addr;

use trust_dns_resolver::Resolver;
use trust_dns_resolver::config::*;

#[macro_use]
extern crate gmod;

#[lua_function]
unsafe fn lookup(lua: gmod::lua::State) -> i32 {
	let hostname = lua.check_string(1);
	match Resolver::new(ResolverConfig::default(), ResolverOpts::default()) {
		Ok(resolver) => {
			match resolver.lookup_ip(hostname.as_ref()) {
				Ok(response) => {
					lua.new_table();
					for (i, addr) in response.iter().enumerate() {
						lua.push_integer(i as isize + 1);
						lua.push_string(addr.to_string().as_ref());
						lua.set_table(-3);
					}
				},
				Err(e) => lua.error(format!("{}", e)),
			}
		},
		Err(e) => lua.error(format!("{}", e)),
	}

	1
}

fn try_parse_ip(ip: &str) -> Option<IpAddr> {
	if let Ok(ipv4) = ip.parse::<Ipv4Addr>(){
		return Some(IpAddr::V4(ipv4));
	}

	if let Ok(ipv6) = ip.parse::<Ipv6Addr>(){
		return Some(IpAddr::V6(ipv6));
	}

	None
}

#[lua_function]
unsafe fn reverse_lookup(lua: gmod::lua::State) -> i32  {
	let ip = lua.check_string(1);
	let parsed_ip = try_parse_ip(ip.as_ref());
	match Resolver::new(ResolverConfig::default(), ResolverOpts::default()) {
		Ok(resolver) if parsed_ip.is_some() => {
			match resolver.reverse_lookup(parsed_ip.unwrap()) {
				Ok(response) => {
					lua.new_table();
					for (i, name) in response.iter().enumerate() {
						lua.push_integer(i as isize + 1);
						lua.push_string(name.to_string().as_ref());
						lua.set_table(-3);
					}
				},
				Err(e) => lua.error(format!("{}", e)),
			}
		},
		Ok(_) => lua.error("invalid ip address"),
		Err(e) => lua.error(format!("{}", e)),
	}

	1
}

#[gmod13_open]
unsafe fn gmod13_open(lua: gmod::lua::State) -> i32 {
	lua.new_table();

	lua.push_function(lookup);
	lua.set_field(-2, lua_string!("Lookup"));

	lua.push_function(reverse_lookup);
	lua.set_field(-2, lua_string!("ReverseLookup"));

	lua.set_global(lua_string!("dns"));

    0
}

#[gmod13_close]
unsafe fn gmod13_close(_: gmod::lua::State) -> i32 {
    0
}