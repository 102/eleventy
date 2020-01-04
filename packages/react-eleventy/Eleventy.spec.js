const React = require("react");
const { render } = require("@testing-library/react");
const Eleventy = require("./Eleventy");

test("renders eleventy", () => {
  const { getByText } = render(React.createElement(Eleventy));
  expect(getByText("110")).not.toBeNull();
});
