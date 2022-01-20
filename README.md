# actix-maud-htmx-h5bp

A template repository implementing [HTML5 Boilerplate 8.0][h5bp] in [Actix] using the [Maud] HTML
template engine. [HTMX] is vendored in for your [HATEOAS] pleasure.

## 🏃 Running

```text
git clone git@github.com:pyxy-dk/actix-maud-htmx-h5bp.git

cd actix-maud-htmx-h5bp

cargo run
```

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
├── 404.html                          ⇏ Implemented in Actix
├── browserconfig.xml                 ⇒ ./src/static/
├── favicon.ico                       ⇒ ./src/static/
├── humans.txt                        ⇒ ./src/static/
├── icon.png                          ⇒ ./src/static/
├── index.html                        ⇏ Implemented in Actix
├── LICENSE.txt                       ⇒ ./LICENSE
├── package.json                      ¬ Not included
├── package-lock.json                 ¬ Not included
├── robots.txt                        ⇒ ./src/static/
├── site.webmanifest                  ⇒ ./src/static/
├── tile.png                          ⇒ ./src/static/
└── tile-wide.png                     ⇒ ./src/static/
```

## 🙏 Thanks to

* The [Actix] web framework.
* [Maud], a Rust macro for writing type-safe HTML. Maud rocks!
* [HTMX] for making frontend fun again.
* Good old [HTML5 Boilerplate][h5bp].

[Actix]: https://actix.rs/
[h5bp]: https://html5boilerplate.com/
[HATEOAS]: https://en.wikipedia.org/wiki/HATEOAS
[HTMX]: https://htmx.org/
[Maud]: https://maud.lambda.xyz/
