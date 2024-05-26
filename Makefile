loader: loader.cpp
	$(CXX) -g -O loader.cpp -Wall -o loader -lbpf

clean:
	rm -rf loader
