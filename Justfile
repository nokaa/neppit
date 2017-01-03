all: build

build: css js
    @cargo build

css:
    @mkdir -p resources
    @sassc -t compressed sass/style.sass resources/style.min.css

js:
    @mkdir -p resources
    @cp javascript/* resources

run:
    @cargo run

clean:
    rm -r resources target

drop-tables:
    @./drop_tables.sh

build-release: css
    @cargo build --release

run-release: build-release
    @cargo run --release
