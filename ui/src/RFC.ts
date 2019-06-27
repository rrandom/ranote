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
  name: string;
  path: string;
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

  public static loadNote(doc: Doc) {
    this.invoke({
      cmd: 'loadNote',
      params: {
        path: doc.path,
      },
      cb: 'loadNoteCb',
    });
  }

  public static saveNote(doc: Doc, content: string) {
    this.invoke({
      cmd: 'saveNote',
      params: {
        file: doc.path,
        content,
      },
    });
  }
}
