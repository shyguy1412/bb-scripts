export function get_all_servers(ns: NS) {
    const servers = ns.scan('home');
    for (const server of servers) {
        const [, ...child_servers] = ns.scan(server);
        servers.push(...child_servers);
    }
    return servers;
}