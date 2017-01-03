all: css

css:
    @mkdir -p resources
    @sassc -t compressed sass/style.sass resources/style.min.css

clean:
    rm -r resources target
