# Chinaroutes (Linux而已)

源repo: https://github.com/fivesheep/chnroutes

这只是上述repo的修改而已

你可以给这程序 http://ftp.apnic.net/apnic/stats/apnic/delegated-apnic-latest 这种档案后指定目标国家代码和Gateway

由于它利用libnl去增加路由 你必须要安装这个程序库

你可以透过这个程序用很短的时间新增上千个路由

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
## 怎么编译

你只需要两种东西安装在你的系统: **Rust程序语言** and **libnl**

我用Ubuntu作为编译平台

```sh
$ apt install libnl-3-dev
$ cd ./chinaroutes
$ cargo build --release
```

然后执行程序于 ./target/release/chinaroutes

结束。
