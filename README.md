<h1 align="center">ReScript React Feather</h1>

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](./LICENSE)

ReScript binding generator for the [react-feather](https://github.com/feathericons/react-feather) library

## Installation
First run the below commands to install the dependencies:

``` sh
yarn add @heliaxdev/rescript-react-feather react-feather
```

Then, add `@heliaxdev/rescript-react-feather` to `bs-dependencies`, i.e.:

``` json
{
  "bs-dependencies": [
    "@rescript/react",
    "@heliaxdev/rescript-react-feather"
  ],
}
```

And you should be good to go.

## Usage
This library can be used in two ways:

### Zero-cost bindings
This library has zero-cost bindings for every icon in the official [TypeScript type definitions](https://github.com/feathericons/react-feather/blob/master/src/index.d.ts).

You can use them like this:

``` rescript
module CancelButton = {
    @react.component
    let make = () => {
        <button ariaLabel="Cancel this, please">
           <ReactFeather.X color="red" size=20 className="my-cross-icon" /> 
        </button>
    }
}
```

### `Icon` component
Aside from the zero-cost bindings, this library also generates an `<Icon />` component that receives an icon name polyvariant as a prop.

It can be used like this:

``` rescript
module ConfirmButton = {
    @react.component
    let make = () => {
        <button ariaLabel="Confirm this, please">
            <ReactFeather.Icon name=#Check color="green" size=20 className="my-check-icon" />
        </button>
    }
}
```

## Props
Except for the extra `name` prop in `ReactFeather.Icon`, all components have the following type:

``` rescript
external make: (~color: string=?, ~size: int=?, ~className: string=?) => React.element
```

