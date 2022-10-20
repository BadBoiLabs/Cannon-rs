
build:
	docker run --rm -v $(shell pwd):/code {{project-name}}/builder

docker_image:
	docker build . -t {{project-name}}/builder

