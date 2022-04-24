# gm_dns
Provides the ability to lookup ips for a hostname and vice-versa.

## Examples
Getting a list of IPs associated with a hostname:
```lua
require("dns")

PrintTable(dns.Lookup("google.com")) -- will return a table of IPs associated with the hostname
```

Getting a list of hostnames associated with an IP:
```lua
require("dns")

PrintTable(dns.Lookup("8.8.8.8")) -- will return a table of hostnames associated with the IP
```

## Compiling
- Open a terminal
- Install **cargo** if you dont have it (on Windows => https://win.rustup.rs) (on Linux/Macos => curl https://sh.rustup.rs -sSf | sh)
- Get [git](https://git-scm.com/downloads) or download the archive for the repository directly
- `git clone https://github.com/Earu/gm_dns` (ignore this if you've downloaded the archive)
- Run `cd gm_dns`
- `cargo build`
- Go in `target/debug` and rename the binary according to your branch and realm (gmsv_dns_win64.dll, gmcl_dns_win64.dll, gmsv_dns_linux.dll, gmcl_dns_linux.dll, gmcl_dns_osx64.dll)
- Put the binary in your gmod `lua/bin` directory

*Note: Even on other platforms than Windows the extension of your modules **needs** to be **.dll***
