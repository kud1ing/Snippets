import { a } from "./A";
import { b } from "./B";

console.log("im Modul");

(<any>window).a = a;
(<any>window).b = b;

export { a, b }