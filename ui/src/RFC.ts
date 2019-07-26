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

interface Note {
  name: string;
  path: string;
  id: string;
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
      cb: 'initCb',
    });
  }

  public static loadNote(noteName: string) {
    this.invoke({
      cmd: 'loadNote',
      params: {
        name: noteName,
      },
      cb: 'loadNoteCb',
    });
  }

  public static saveNote(note: Note, content: string) {
    this.invoke({
      cmd: 'saveNote',
      params: {
        name: note.name,
        content,
      },
    });
  }

  public static newNote() {
    console.log('newNote');
    this.invoke({
      cmd: 'newNote',
      cb: 'newNoteCb',
    });
  }

  public static debug(info: any) {
    const msg = JSON.stringify(info);
    window.debugCb = () => ({});

    this.invoke({
      cmd: 'debug',
      params: {
        msg,
      },
      cb: 'debugCb',
    });
  }
}
