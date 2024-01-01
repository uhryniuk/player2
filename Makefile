docker:
	docker build -f docker/client/Dockerfile .
	docker build -f docker/server/Dockerfile .

compose: docker
	docker compose up
