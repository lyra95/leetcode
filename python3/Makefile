.PHONY: test
test:
	pytest sol/*.py


.PHONY: lint
lint: pylint flake8

.PHONY: pylint
pylint:
	pylint sol/*.py --rcfile=./.pylintrc


.PHONY: flake8
flake8:
	flake8 sol/*.py
