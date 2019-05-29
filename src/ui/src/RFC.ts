declare global {
  interface External {
    invoke: (command: string) => void;
  }
}

interface Command {
  cmd: string;
  params?: object;
}

export default class RFC {
  public static invoke(command: Command) {
    external.invoke(JSON.stringify(command));
  }
}
