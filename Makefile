build:
	rustc $(name).rs && ./$(name) && rm $(name)
