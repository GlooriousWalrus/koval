## leptos-csr-tailwind

Use as a starter for a client side rendered web app. There is no server side rendering setup in this repository to keep it simple.

```bash
cargo leptos new --git https://github.com/talves/leptos-csr-tailwind
```

The above command will do a shallow copy of the repository and prompt you for a name for the project folder.

You will have to have `cargo-leptos` installed to run the command above, but we are not using cargo leptos for our dev and build. 

We will be using `trunk`, so make sure to have `cargo install trunk` installed to use the commands.

Yarn is being used, so we can take full advantage of the tailwind cli.

### Development

```bash
yarn dev
```

### Tailwind

There are 2 modes for using Trunk and Tailwind together and they will decide how you setup your build process. Both are valid ways to process the css file, but the second option `(2)` will give us more control over our tailwind compile and we will use it here. Yarn is optional, but it's what we are using in our workflow for the repository. It would work just as well using `npm` or `pnpm` also.

#### Option 1 (Simplified)

Using an input file for our `tailwind-css` will tell trunk to download the tailwindcss binary and compile the input to our dist folder. In this case, we don't need to run a tailwind command to build the css for us.

```html
    <link data-trunk rel="tailwind-css" href="style/input.css" />
```

#### Option 2

We'll use a compiled output file for our `css` link and tell trunk to use it. 

```html
    <link data-trunk rel="css" href="style/output.css" />
```

This will allow us to run a pre-build `tailwind` command with more control over the options we want like minify in our case. We create a `Trunk.toml` file with a pre-build hooks command to run when we are developing and releasing our site.

```bash
tailwindcss -i style/input.css -o style/output.css --minify
```

This option allows you to do what you would like and have more control over the commands. You could also remove the `Trunk.toml` and run the commands in their own processes also. Debugging without minify would be nice, but I only do it when there's a need. I like using what's going to be in production, but there are drawbacks to both. Your choice!

### Deployment

There are a lot of hosting providers out there and since we are using Rust and Leptos (CSR) Client Side Rendering we can pick one that supports both.

#### Netlify

We will use a shell command line to check for a cached `trunk` install located at `bin/install_trunk.sh`.

```bash
chmod 744 ./bin/install_trunk.sh && TRUNK_VERSION=0.17.5 ./bin/install_trunk.sh
```

`netlify.toml` is setup to run the package scripts with the setups for the deploy

```toml
[build]
publish = "site/dist/"
command = "yarn install_trunk && yarn build"
```
