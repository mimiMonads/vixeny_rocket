import {Hono} from "hono"

const app = new Hono();

app.get("/", (c) => c.text("hello world"));
app.get("/a/b/c/d/e/f", (c) => {
  return c.text("You've reached /a/b/c/d/e/f!");
});
app.get("/get/:id", (c) => {
  return c.text( "Recived id: " + c.req.param("id"));
});
app.get("/get/:name/:id/:number", (c) => {
  return c.text("Name: " + c.req.param("name") + " , ID: " + c.req.param("id") + " , Number: "  +c.req.param("c"));
});
app.get("/query", (c) => {
  return c.text("Recived q :" +  c.req.query("q") || "");
});

export default {
  port: 8000,
  hostname: "127.0.0.1",
  fetch: app.fetch

}
