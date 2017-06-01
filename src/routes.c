#include <netlink/netlink.h>
#include <netlink/socket.h>
#include <netlink/types.h>
#include <netlink/addr.h>
#include <netlink/route/route.h>
#include <strings.h>

struct route_entry
{
    char dst[4];
    unsigned int mask;
};

int netlink_route_connect(void **nl_sk)
{
    struct nl_sock *sk;
    sk = nl_socket_alloc();
    int ret = nl_connect(sk, NETLINK_ROUTE);
    if (ret) {
        nl_socket_free(sk);
        return ret;
    }
    *nl_sk = sk;
    return 0;
}

void netlink_route_disconnect(void *nl_sk)
{
    struct nl_sock *sk = (struct nl_sock *)nl_sk;
    nl_close(sk);
    nl_socket_free(sk);
}

static inline struct nl_addr *construct_addr(const struct route_entry *route)
{
    struct nl_addr *addr = nl_addr_build(AF_INET, route->dst, sizeof(route->dst));
    nl_addr_set_prefixlen(addr, route->mask);
    return addr;
}

int netlink_route_add(void *nl_sk, unsigned int table, const char via[4], const struct route_entry *route)
{
    struct rtnl_route *rtroute = rtnl_route_alloc();
    bzero(rtroute, sizeof(rtroute));
    struct nl_addr *dst = construct_addr(route);
    rtnl_route_set_table(rtroute, table);
    rtnl_route_set_family(rtroute, AF_INET);
    rtnl_route_set_dst(rtroute, dst);

    struct rtnl_nexthop *routenh = rtnl_route_nh_alloc();
    struct nl_addr* viaaddr = nl_addr_build(AF_INET, via, 4);
    rtnl_route_nh_set_gateway(routenh, viaaddr);
    rtnl_route_add_nexthop(rtroute, routenh);

    int ret = rtnl_route_add((struct nl_sock*)nl_sk, rtroute, 0);

    return ret;
}

int netlink_route_del(void *nl_sk, unsigned int table, const char via[4], const struct route_entry *route)
{
    struct rtnl_route *rtroute = rtnl_route_alloc();
    bzero(rtroute, sizeof(rtroute));
    struct nl_addr *dst = construct_addr(route);
    rtnl_route_set_table(rtroute, table);
    rtnl_route_set_family(rtroute, AF_INET);
    rtnl_route_set_dst(rtroute, dst);

    struct rtnl_nexthop *routenh = rtnl_route_nh_alloc();
    struct nl_addr* viaaddr = nl_addr_build(AF_INET, via, 4);
    rtnl_route_nh_set_gateway(routenh, viaaddr);
    rtnl_route_add_nexthop(rtroute, routenh);

    int ret = rtnl_route_delete((struct nl_sock*)nl_sk, rtroute, 0);

    return ret;
}
