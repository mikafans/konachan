# hypothesis

a tool for downloading yandre/konachan stuffs

## TODO

- how to make download thread running in concurrent

## Problem Track

Simply using `reqwest::Proxy::all("socks5://xxx")` couldn't reach the target service, always stoped at tls error or hand shake eof. By checking on repo issues, found that should use the protocol `socks5h` instead of the origin, which also indicates doing DNS resolution on the proxy. Ref [unable to make socks5 to work](https://github.com/seanmonstar/reqwest/issues/1625)
