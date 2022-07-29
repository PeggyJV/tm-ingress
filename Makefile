.DEFAULT_GOAL := build

INGRESS_RPC_PORT := 26655
NAME := "tmingress"

build:
	@docker build -t $(NAME):prebuilt -f Dockerfile .

clean:
	@docker rm --force \
	    $(shell docker ps -qa --filter="name=$(NAME)") \
	    $(shell docker ps -qa --filter="name=happy-path") \
	    1>/dev/null \
	    2>/dev/null \
	    || true
	@docker wait \
	    $(shell docker ps -qa --filter="name=$(NAME)") \
	    $(shell docker ps -qa --filter="name=happy-path") \
	    1>/dev/null \
	    2>/dev/null \
	    || true
	@docker network prune --force 1>/dev/null 2>/dev/null || true

test:
	@mkdir -p ./test_logs
	@cargo test happy_path -- --nocapture || make -s fail

fail:
	@echo 'test failed; dumping container logs into ./test_logs for review'
	@docker logs $(NAME) > ./test_data/$(NAME).log
	@false
