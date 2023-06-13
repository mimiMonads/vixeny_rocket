export default{
    port: 8000, 
    hostname: "127.0.0.1",
    fetch: await (
        // deno-lint-ignore require-await
        async fun => fun.default({ hasName: "http://127.0.0.1:8000/" })([
         {path: "/", type: "response",r: () => new Response("Hello world")},
         {path: "/get/:id", f: f => "Received id : " + f.param.id},
          {path:"/get/:name/:id/:number", f: f => "Name: " + f.param.a + ", ID: " + f.param.b + " Num: " + f.param.c},
         {path: "/query", query:{only:["q"]},f: f => "Received q : " + f.query?.q },
         {path: "/a/b/c/d/e/f", f: () => "You've reached /a/b/c/d/e/f!"}
     ])
     )(
         await import("vixeny")
     )
}

