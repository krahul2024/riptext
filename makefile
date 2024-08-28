BINARY_NAME=rt

SRC=$(wildcard *.go)

build: $(SRC)
	@echo "Preparing the riptext tool..."
	@go build -o $(BINARY_NAME)

run: build
	@echo "Starting riptext..."
	@./$(BINARY_NAME)

clean:
	@echo "Cleaning the mess..."
	@rm -f $(BINARY_NAME)

tidy:
	@echo "Tidying the deps..."
	@go mod tidy

fmt:
	@echo "Beautifying the code..."
	@go fmt ./...

.PHONY: build run clean tidy fmt

