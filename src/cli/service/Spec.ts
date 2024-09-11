export type Section = keyof Service;
export type UnitDirective = keyof Service['Unit'];
export type InstallDirective = keyof Service['Install'];
export type ServiceDirective = keyof Service['Service'];
export type SocketDirective = keyof Service['Socket'];
export type PathDirective = keyof Service['Path'];
export type ServiceType = keyof Service['Service']['Type'];


export type Service = {
  Unit: {
    Description: string;
    Documentation: string;
    Requires: string;
    Wants: string;
    BindsTo: string;
    Before: string;
    After: string;
    Conflicts: string;
  };
  Install: {
    WantedBy: string;
    RequiredBy: string;
    Alias: string;
    Also: string;
    DefaultInstance: string;
  };
  Service: {
    Type: 'simple' | 'forking' | 'oneshot' | 'dbus' | 'notify' | 'idle';
    RemainAfterExit: 'true' | 'false';
    PIDFile: string;
    BusName: string;
    NotifyAccess: string;
    ExecStart: string;
    ExecStartPre: string;
    ExecStartPost: string;
    ExecReload: string;
    ExecStop: string;
    ExecStopPost: string;
    RestartSec: string;
    Restart: string;
    TimeoutSec: string;
  };
  Socket: {
    ListenStream: string;
    ListenDatagram: string;
    ListenSequentialPacket: string;
    ListenFIFO: string;
    Accept: string;
    SocketUser: string;
    SocketGroup: string;
    SocketMode: string;
    Service: string;
  };
  Path: {
    PathExists: string;
    PathExistsGlob: string;
    PathChanged: string;
    PathModified: string;
    DirectoryNotEmpty: string;
    Unit: string;
    MakeDirectory: string;
  };
};  