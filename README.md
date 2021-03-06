# rocket-maud-htmx-h5bp

A template repository implementing [HTML5 Boilerplate 8.0][h5bp] in [Rocket] using the [Maud] HTML
template engine. [HTMX] is vendored in for your [HATEOAS] pleasure.

## β  Caution

[Rocket] is currently (late 2021) in [a bit of a hiatus](https://github.com/SergioBenitez/Rocket/discussions/1672#discussioncomment-1217547).
This template is built on `rocket-0.5.0-rc.1` which has major updates to async handling.
Unfortunately, the RC status means that [Maud] does not support this Rocket version out of the box,
so the template also relies on [a draft PR with Rocket-0.5.0 support](https://github.com/lambda-fairy/maud/pull/307) and *nightly* Rust.

Taken together, this template isn't at its most stable right now, but it seems to work fine. When
the projects regain traction the template will of course be updated.

## π Running

```text
git clone git@github.com:pyxy-dk/rocket-maud-htmx-h5bp.git

cd rocket-maud-htmx-h5bp

rustup override set nightly

cargo run
```

## πΊοΈ File mapping from H5BP

The files from a standard download of H5BP 8.0 maps to the following files in
this template project:

```text
h5bp
β
βββ css
β   βββ main.css                      β ./src/static/css/
β   βββ normalize.css                 β ./src/static/css/
β
βββ doc                               Β¬ Not included
β
βββ img                               β ./src/static/img/
β
βββ js
β   βββ vendor
β   β   βββ modernizer-3.11.2.min.js  β ./src/static/js/vendor/
β   βββ main.js                       β ./src/static/js/
β   βββ plugins.js                    β ./src/static/js/
β
βββ .editorconfig                     β expanded in ./.editorconfig
βββ .gitattributes                    β expanded in ./.gitattributes
βββ .gitignore                        β expanded in ./.gitignore
βββ .htaccess                         Β¬ Not included
βββ 404.html                          β Implemented in Rocket
βββ browserconfig.xml                 β ./src/static/
βββ favicon.ico                       β ./src/static/
βββ humans.txt                        β ./src/static/
βββ icon.png                          β ./src/static/
βββ index.html                        β Implemented in Rocket
βββ LICENSE.txt                       β ./LICENSE
βββ package.json                      Β¬ Not included
βββ package-lock.json                 Β¬ Not included
βββ robots.txt                        β ./src/static/
βββ site.webmanifest                  β ./src/static/
βββ tile.png                          β ./src/static/
βββ tile-wide.png                     β ./src/static/
```

## π Thanks to

* The [Rocket] web framework.
* [Maud], a Rust macro for writing type-safe HTML. Maud rocks!
* [HTMX] for making frontend fun again.
* Good old [HTML5 Boilerplate][h5bp].

[h5bp]: https://html5boilerplate.com/
[HATEOAS]: https://en.wikipedia.org/wiki/HATEOAS
[HTMX]: https://htmx.org/
[Maud]: https://maud.lambda.xyz/
[Rocket]: https://rocket.rs/
