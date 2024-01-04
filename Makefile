.PHONY: build help fetch read add list export clear

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

export:
	if [ ! -f "$(PWD)/items.yaml" ]; then \
		echo "Error: items.yaml not found in $(PWD)."; \
		exit 1; \
	fi
	if [ ! -f "$(PWD)/list.yaml" ]; then \
		echo "Error: list.yaml not found in $(PWD)."; \
		exit 1; \
	fi
	docker run --rm \
		-v gust_data:/app \
		-v $(PWD)/items.yaml:/app/items.yaml \
		-v $(PWD)/list.yaml:/app/list.yaml \
		gust \
		export

clear:
	docker run --rm -v gust:/app gust update list clear
