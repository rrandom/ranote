declare global {
  interface External {
    invoke: (command: string) => void;
  }
}

interface Command {
  cmd: string;
  params?: object;
  cb?: string;
}

export default class RFC {
  public static invoke(command: Command) {
    if (external) {
      command = { ...command, ...command.params };
      external.invoke(JSON.stringify(command));
    }
  }

  public static testClick() {
    this.invoke({
      cmd: 'testClick',
      cb: 'testClickCb',
    });
  }

  public static init() {
    this.invoke({
      cmd: 'init',
    });
  }

  public static loadFile(fileName: string) {
    this.invoke({
      cmd: 'loadFile',
      params: {
        fileName,
      },
      cb: 'loadFileCb',
    });
  }

  public static saveFile(file: string, contents: string) {
    this.invoke({
      cmd: 'saveFile',
      params: {
        file, contents,
      },
    });
  }
}
