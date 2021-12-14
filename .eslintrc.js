import "@rushstack/eslint-patch/modern-module-resolution";

export const root = true;
export const ignorePatterns = ["dist/", "*.js"];
export const parserOptions = {
  tsconfigRootDir: __dirname,
  project: "tsconfig.json",
};
export const env = {
  node: true,
};
