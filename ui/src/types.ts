export interface Note {
  name: string;
  path: string;
  id: string;
}

export interface ActiveNote extends Note {
  content: string;
}

export interface Command {
  cmd: string;
  params?: object;
  cb?: string;
}
