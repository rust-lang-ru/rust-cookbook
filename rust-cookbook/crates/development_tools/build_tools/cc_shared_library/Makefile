CXX = g++
CXXFLAGS = -std=c++11 -Wall -Werror

all: $(OUT_DIR)/libmylibrary.so

$(OUT_DIR)/libmylibrary.so: src/mylibrary.cc src/mylibrary.h
	# Position-independent code (`-fpic`) is required for shared libraries.
	$(CXX) $(CXXFLAGS) -fpic -c src/mylibrary.cc -o $(OUT_DIR)/mylibrary.o
	$(CXX) -shared -o $(OUT_DIR)/libmylibrary.so $(OUT_DIR)/mylibrary.o
	rm $(OUT_DIR)/mylibrary.o
