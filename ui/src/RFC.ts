declare global {
  interface External {
    invoke: (command: string) => void;
  }
}

import { Note, ActiveNote, Command } from './types';

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

  public static saveNote(note: Note|ActiveNote, newContent: string) {
    this.invoke({
      cmd: 'saveNote',
      params: {
        id: note.id,
        content: newContent,
      },
    });
  }

  public static loadNote(noteId: string) {
    this.invoke({
      cmd: 'loadNote',
      params: {
        id: noteId,
      },
      cb: 'loadNoteCb',
    });
  }

  public static newNote() {
    this.invoke({
      cmd: 'newNote',
      cb: 'newNoteCb',
    });
  }

  public static debug(name: string, info: any) {
    const msg = JSON.stringify({ info });
    window.debugCb = () => ({});

    this.invoke({
      cmd: 'debug',
      params: {
        name,
        msg,
      },
      cb: 'debugCb',
    });
  }
}
