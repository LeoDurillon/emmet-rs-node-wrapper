import test from "ava";

import { expand } from "../index.js";

test("correctly working", (t) => {
  t.is(expand("html"), "<html></html>");
});
