import { DynamicNS } from '@/lib/dynamic';

export function getAllServers(dynamic: DynamicNS): string[] {
    const servers = dynamic('scan', ['home']);

    for (const server of servers) {
        const newServers = dynamic('scan', [server]).slice(1);

        servers.push(...newServers.filter((s) => !servers.includes(s)));
    }
    return servers;
}
