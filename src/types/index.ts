export interface ServerConfig {
  id: string;
  name: string;
  group: string;
  host: string;
  port: number;
  username: string;
  authType: "password" | "keyfile" | "agent";
  encryptedPassword?: string;
  keyFilePath?: string;
  tags: string[];
  createdAt: number;
  updatedAt: number;
}

export interface TerminalTab {
  id: string;
  serverId: string;
  title: string;
  ptyId: number;
}

export interface SftpFile {
  name: string;
  path: string;
  isDirectory: boolean;
  size: number;
  modified: number;
  permissions: string;
}

export interface TransferProgress {
  file: string;
  total: number;
  transferred: number;
  speed: number;
}
