import { Command, createCommand } from "@/lib/Commander";
import { ScriptArg } from "NetscriptDefinitions";

type ServiceCommand = 'start' | 'stop' | 'enable' | 'disable' | 'status' | 'dashboard';
type CommandMap = {
  [key in ServiceCommand]: () => Command
};

const commands: CommandMap = {
  start: () => {
    return createCommand('start')
      .action((ns) => {
        throw new Error("Function not implemented.");
      });
  },
  stop: () => {
    return createCommand('stop')
      .action((ns) => {
        throw new Error("Function not implemented.");
      });
  },
  enable: () => {
    return createCommand('enable')
      .action((ns) => {
        throw new Error("Function not implemented.");
      });
  },
  disable: () => {
    return createCommand('disable')
      .action((ns) => {
        throw new Error("Function not implemented.");
      });
  },
  status: () => {
    return createCommand('status')
      .action((ns) => {
        throw new Error("Function not implemented.");
      });
  },
  dashboard: () => {
    return createCommand('dashboard')
      .action((ns) => {
        throw new Error("Function not implemented.");
      });
  }
};

export const cli = () => {
  const program = createCommand();
  for (const command in commands) {
    program.addCommand(commands[command as ServiceCommand]());
  }
  return program;
};