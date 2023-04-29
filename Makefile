.PHONY: outputs round-robin

define banner
	@printf "\n"
	@printf "$$(tput bold)********************************************************************************$$(tput sgr0)\n"
	@string="$(1)" && printf "$$(tput bold)* %-$$((76))s *\n" "$$string"
	@printf "$$(tput bold)********************************************************************************$$(tput sgr0)\n"
	@printf "\n"
endef

outputs:
	rm -rf outputs
	# round robin
	SCHEDULER=round-robin WRITE_OUTPUT=true cargo test
	SCHEDULER=round-robin WRITE_OUTPUT=true TIMESLICE=5 REMAINING=2 cargo test
	SCHEDULER=round-robin WRITE_OUTPUT=true TIMESLICE=3 REMAINING=3 cargo test

round-robin:
ifndef TEST
	$(error No test defined)
endif
	$(call banner,Round Robin Timeslice: 3 Remaining: 1)
	SCHEDULER=round-robin cargo test $(TEST) -q
	$(call banner,Round Robin Timeslice: 5 Remaining: 2)
	SCHEDULER=round-robin TIMESLICE=5 REMAINING=2 cargo test $(TEST) -q
	$(call banner,Round Robin Timeslice: 3 Remaining: 3)
	SCHEDULER=round-robin TIMESLICE=3 REMAINING=3 cargo test $(TEST) -- -q