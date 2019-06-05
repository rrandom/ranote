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
    external.invoke(JSON.stringify(command));
  }

  public static testClick() {
    this.invoke({
      cmd: 'testClick',
      cb: 'testClickCb',
    });
  }
}
