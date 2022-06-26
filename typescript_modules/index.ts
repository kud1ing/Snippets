import { a } from "./A";
import { b } from "./B";

console.log("in `index.ts`");

(<any>window).a = a;
(<any>window).b = b;

export { a, b }