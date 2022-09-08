<a href="https://www.linkedin.com/in/ahmed-hazem-b50124215">
    <img src="https://raw.githubusercontent.com/hazemeldoc/subdomain-brute/master/img/logo.svg" align="right" height="60" />
</a>

# subdomain brutforce in rust ![GitHub last commit](https://img.shields.io/github/last-commit/hazemeldoc/subdomain-brute?style=plastic)

> an active subdomain enum tool based on research on [NOERROR & NODATA](https://www.securesystems.de/blog/enhancing-subdomain-enumeration-ents-and-noerror/)

a simple multithreaded tool to preform active domain enumeration, unlike some other tools this tool doesn't ignore NODATA responses returned from DNS server instead it add them to queue to re-scan them

## ENT
<img src="https://github.com/hazemeldoc/subdomain-brute/raw/master/img/DNS_hierarchy.png" align="right" width="400" height="200"/>
also known as empty non-terminals , this response is the result of the following scenario
-we have a domain called example.com
-then we decided to add a subdomain called blog.dev.example.com
-despite not adding a dev.example.com entry it's automatically created
-in this case dev.exaple.com return NODATA response



>in order to make out the most of this tool always use -r option which will rescan ENT (if it found dev.example.com (ENT) it will bruteforce it again which will result in finding blog.dev.com 

## usage

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

## DNS 

-the script use cloudflare DNS server

![code_snippet](https://raw.githubusercontent.com/hazemeldoc/subdomain-brute/master/img/code.svg)

but you could change this to desired DNS server by changing the ip in this code snippet and it should work fine

## installation
```
git clone https://github.com/hazemeldoc/subdomain-brute.git
cd subdomain-brute
cargo build
```

## sample run
![RUN](https://github.com/hazemeldoc/subdomain-brute/raw/master/img/github.gif)
