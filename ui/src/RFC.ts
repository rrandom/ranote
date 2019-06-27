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

interface Doc {
  name: string,
  path: string,
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

  public static loadFile(doc: Doc) {
    this.invoke({
      cmd: 'loadFile',
      params: {
        fileName: doc.path,
      },
      cb: 'loadFileCb',
    });
  }

  public static saveFile(doc: Doc, contents: string) {
    this.invoke({
      cmd: 'saveFile',
      params: {
        file: doc.path,
        contents,
      },
    });
  }
}
