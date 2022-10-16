# rocket-maud-htmx-h5bp

A template repository implementing [HTML5 Boilerplate 8.0][h5bp] in [Rocket] using the [Maud] HTML
template engine. [HTMX] is vendored in for your [HATEOAS] pleasure.

## ⚠ Caution

[Rocket] is currently (late 202~~1~~2) in [a bit of a hiatus](https://github.com/SergioBenitez/Rocket/discussions/1672#discussioncomment-1217547).
This template is built on `rocket-0.5.0-rc.2` which has major updates to async handling.
Unfortunately, the RC status means that [Maud] does not support this Rocket version out of the box,
so the template also relies on [a draft PR with Rocket-0.5.0 support](https://github.com/lambda-fairy/maud/pull/353).

Taken together, this template isn't at its most stable right now, but it seems to work fine. When
the projects regain traction the template will of course be updated.

## 🏃 Running

```text
git clone git@github.com:pyxy-dk/rocket-maud-htmx-h5bp.git

cd rocket-maud-htmx-h5bp

cargo run
```

Now open <http://localhost:8000/> in your browser.

## 🗺️ File mapping from H5BP

The files from a standard download of H5BP 8.0 maps to the following files in
this template project:

```text
h5bp
│
├── css
│   ├── main.css                      ⇒ ./src/static/css/
│   └── normalize.css                 ⇒ ./src/static/css/
│
├── doc                               ¬ Not included
│
├── img                               ⇒ ./src/static/img/
│
├── js
│   ├── vendor
│   │   └── modernizer-3.11.2.min.js  ⇒ ./src/static/js/vendor/
│   ├── main.js                       ⇒ ./src/static/js/
│   └── plugins.js                    ⇒ ./src/static/js/
│
├── .editorconfig                     ⇒ expanded in ./.editorconfig
├── .gitattributes                    ⇒ expanded in ./.gitattributes
├── .gitignore                        ⇒ expanded in ./.gitignore
├── .htaccess                         ¬ Not included
├── 404.html                          ⇏ Implemented in Rocket
├── browserconfig.xml                 ⇒ ./src/static/
├── favicon.ico                       ⇒ ./src/static/
├── humans.txt                        ⇒ ./src/static/
├── icon.png                          ⇒ ./src/static/
├── index.html                        ⇏ Implemented in Rocket
├── LICENSE.txt                       ⇒ ./LICENSE
├── package.json                      ¬ Not included
├── package-lock.json                 ¬ Not included
├── robots.txt                        ⇒ ./src/static/
├── site.webmanifest                  ⇒ ./src/static/
├── tile.png                          ⇒ ./src/static/
└── tile-wide.png                     ⇒ ./src/static/
```

## 🙏 Thanks to

* The [Rocket] web framework.
* [Maud], a Rust macro for writing type-safe HTML. Maud rocks!
* [HTMX] for making frontend fun again.
* Good old [HTML5 Boilerplate][h5bp].

[h5bp]: https://html5boilerplate.com/
[HATEOAS]: https://en.wikipedia.org/wiki/HATEOAS
[HTMX]: https://htmx.org/
[Maud]: https://maud.lambda.xyz/
[Rocket]: https://rocket.rs/
