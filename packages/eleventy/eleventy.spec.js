const { eleventy } = require("./");

test("returns eleventy", () => {
  expect(eleventy).toEqual(110);
});
