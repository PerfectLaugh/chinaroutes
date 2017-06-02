# Chinaroutes (for Linux-only)

Original repo: https://github.com/fivesheep/chnroutes

This is only the modification from above project.

You can feed this program the file like http://ftp.apnic.net/apnic/stats/apnic/delegated-apnic-latest then specify the country (case-sensitive) and the target router.

Because it add routes through libnl, you must have libnl in your system.

You can almost instantaneously add a bunch of country ips' route by this program.

```
user@Dev:~/chinaroutes$ ./target/release/chinaroutes --help
chinaroutes 0.1.2
Add country routes for linux routing table

USAGE:
    chinaroutes [FLAGS] [OPTIONS] --country <COUNTRY> --file <FILE> --target <GATEWAY>

FLAGS:
        --delete     Delete the entries (Default to add)
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --country <COUNTRY>    Target country (case-sensitive)
    -f, --file <FILE>          Input asn-delegated file
    -t, --table <TABLE>        Routing table (default: 254)
        --target <GATEWAY>     Target gateway
```
