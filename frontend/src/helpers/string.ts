export const formatString = (s: string, ...args: string[]): string =>
    args.reduce((acc, arg) => acc.replace("{}", arg), s);
