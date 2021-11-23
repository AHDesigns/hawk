import { bar } from "./foo";
console.log("hello world!");

function foo(n: number): number {
  console.log(bar());

  return n + 4;
}

foo(3);
