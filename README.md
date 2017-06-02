# Chinaroutes (for Linux-only)

Original repo: https://github.com/fivesheep/chnroutes

This is only the modification from above project.

You can feed this program http://ftp.apnic.net/apnic/stats/apnic/delegated-apnic-latest then specify the country (apnic only currently) and the target router.

Because it add routes through libnl, you must have libnl in your system.

You can almost instantaneously add a bunch of country ips' route by this program.

```
chinaroutes 0.1.1
PerfectLaugh
Add china routes for personal routing

USAGE:
    chinaroutes [FLAGS] [OPTIONS] --country <COUNTRY> --file <FILE> --target <ROUTER>

FLAGS:
        --delete     Delete the entries (Default to add)
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --country <COUNTRY>    Target country (case-sensitive)
    -f, --file <FILE>          Input/Output file
    -t, --table <TABLE>        Routing table
        --target <ROUTER>      Target router
```
