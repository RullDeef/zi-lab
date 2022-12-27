CXX := g++
CXXFLAGS := -std=c++23 -Wall -Wno-sign-compare -Werror -g -ggdb
SRC := $(shell find src -name "*.cpp")
HEADERS := $(shell find src -name "*.h")

app: $(SRC) $(HEADERS)
	$(CXX) $(CXXFLAGS) -o $@ $(SRC)

clean:
	rm -rf app
