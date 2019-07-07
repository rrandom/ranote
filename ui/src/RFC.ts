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

  public static loadNote(note: Note) {
    this.invoke({
      cmd: 'loadNote',
      params: {
        name: note.name,
      },
      cb: 'loadNoteCb',
    });
  }

  public static saveNote(note: Note, content: string) {
    console.log('note', note);
    this.invoke({
      cmd: 'saveNote',
      params: {
        name: note.name,
        content,
      },
    });
  }
}
