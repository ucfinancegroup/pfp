import React from 'react';
import { render } from '@testing-library/react';
import Header from "./Header";

test('Logo should be rendered', () => {
    const output = render(<Header />);

    expect(output.container.querySelector("img#logo")).toBeTruthy();
});
