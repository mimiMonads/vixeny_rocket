import fun from "./fun.ts"

Bun.serve({
    port: 8000, 
    hostname: "127.0.0.1",
    fetch: fun({ hasName: "http://127.0.0.1:8000/" }
        )([
            {path: "/", f: () => "Welcome to the home page!"},
            {path: "/get/:id", f: f => "Received id : " + f.param.id},
            {path: "/query", query:{only:["q"]},f: f => "Received query : " + f.query?.q },
            {path: "/a/b/c/d/e/f", f: () => "You've reached /a/b/c/d/e/f!"}
        ])
})
