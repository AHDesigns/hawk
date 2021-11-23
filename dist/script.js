(() => {
  // src/bar.ts
  var foo = () => {
    console.log("fe");
    console.log("fo");
    console.log("fum");
  };

  // src/foo.ts
  function bar() {
    foo();
    return "";
  }

  // src/index.ts
  console.log("hello world!");
  function foo2(n) {
    console.log(bar());
    return n + 4;
  }
  foo2(3);
})();
//# sourceMappingURL=script.js.map
