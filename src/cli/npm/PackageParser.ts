export type PackageJson = Partial<{
  name: string;
  main: string;
  browser: string;
  exports: string | {
    [path: string]: string | {
      [condition: string]: string | {
        [method: string]: string;
      };
    };
  };
  imports: {
    [path: string]: string | {
      [condition: string]: string | {
        [method: string]: string;
      };
    };
  };
}>;