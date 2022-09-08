<a href="https://www.linkedin.com/in/ahmed-hazem-b50124215">
    <img src="https://raw.githubusercontent.com/hazemeldoc/subdomain-brute/master/img/logo.svg" align="right" height="60" />
</a>

# subdomain brutforce in rust ![GitHub last commit](https://img.shields.io/github/last-commit/hazemeldoc/subdomain-brute?style=plastic)
> an active subdomain enum tool based on research on [NOERROR & NODATA](https://www.securesystems.de/blog/enhancing-subdomain-enumeration-ents-and-noerror/)
a simple multithreaded tool to preform active domain enum unlike other tools this tool doesn't ignore NODATA responses returned from DNS server instead it add them to queue to re-scan them

##usage

```
USAGE:
    subdomains_brute <target> [FLAGS]

FLAGS:
    -w, --wordlist          the path to wordlist
    -c, --Concurrency       the number of concurent requests
    -r, --recursive         re-scan ENT nodes

ARGS:
    <target>    The target to scan (e.g:google.com)
```

##DNS 

-the script use cloudflare DNS server
(https://raw.githubusercontent.com/hazemeldoc/subdomain-brute/master/img/code.svg)
but you could change this to desired DNS server by changing the ip in this code snippet and it should work fine

##installation
```
git clone https://github.com/hazemeldoc/subdomain-brute.git
cd subdomain-brute
cargo build
```
