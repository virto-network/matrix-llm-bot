build:
	docker build -t matrix-bot -f Dockerfile .
run:
	docker run -d -p 80:8000 matrix-bot