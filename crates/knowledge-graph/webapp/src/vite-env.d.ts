/// <reference types="vite/client" />
/// <reference types="vite-plugin-svgr/client" />

/** Allow TypeScript to import *.module.css files as plain style objects. */
declare module '*.module.css' {
  const styles: { readonly [className: string]: string };
  export default styles;
}
