run:
	RUST_ENV=debug cargo run

buildrun:
	cd client && npm run build &&	cd - && sass --update --no-source-map static/scss/custom:static/css && RUST_ENV=debug cargo run

install:
	npm -g install sass
