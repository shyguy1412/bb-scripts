export type PackageJson = {
  name: string;
  main: string;
  exports: string | {
    [path: string]: string | {
      [condition: string]: string;
    };
  };
};