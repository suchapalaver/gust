.PHONY: build help fetch read add list clear

build:
	docker build --tag gust --file Dockerfile .

help:
	docker run --rm gust
	# or
	# cargo run -- -h

fetch:
	@if [ -z "$(url)" ]; then \
		echo "Please provide a URL using 'make fetch url=<your_url_here>'"; \
	else \
		docker run --rm -v gust:/app gust fetch --url "$(url)"; \
	fi

read:
	@if [ -z "$(recipe)" ]; then \
		docker run --rm -v gust:/app gust read recipes; \
	else \
		docker run --rm -v gust:/app gust read --recipe "$(recipe)"; \
	fi

add:
	@if [ -z "$(recipe)" ]; then \
		echo "Please provide a recipe name using 'make add recipe=<your_recipe_here>'"; \
	else \
		docker run --rm -v gust:/app gust add list --recipe "$(recipe)"; \
	fi

list:
	docker run --rm -v gust:/app gust read list

clear:
	docker run --rm -v gust:/app gust update list clear
