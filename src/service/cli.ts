import { ScriptArg } from "NetscriptDefinitions";

export type ValidCommands = 'start' | 'stop' | 'enable' | 'disable' | 'status' | 'help' | 'dashboard';
export type CommandMap = {
  [key in ValidCommands]: (ns: NS, input: ScriptArg[]) => Promise<void>
};

export const Commands: CommandMap = {
  start: async (ns, input) => {
    throw new Error("Function not implemented.");
  },
  stop: async (ns, input) => {
    throw new Error("Function not implemented.");
  },
  enable: async (ns, input) => {
    throw new Error("Function not implemented.");
  },
  disable: async (ns, input) => {
    throw new Error("Function not implemented.");
  },
  status: async (ns, input) => {
    throw new Error("Function not implemented.");
  },
  help: async (ns, input) => {
    throw new Error("Function not implemented.");
  },
  dashboard: async (ns, input) => {
    throw new Error("Function not implemented.");
  }
};