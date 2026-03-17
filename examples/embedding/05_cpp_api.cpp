/* examples/embedding/05_cpp_api.cpp
 * Example: Using Taurine from C++
 *
 * Note: This is a demonstration of the C++ wrapper structure.
 *
 * Compile with:
 *   g++ -o 05_cpp_api 05_cpp_api.cpp -ltaurine -std=c++11
 */

#include <iostream>
#include <string>
#include <stdexcept>

// Forward declarations (would be in taurine.h)
typedef struct TaurineVM TaurineVM;

extern "C" {
    extern TaurineVM* taurine_new(void);
    extern void taurine_free(TaurineVM* vm);
    extern int taurine_run(TaurineVM* vm, const char* code);
    extern const char* taurine_get_error(TaurineVM* vm);
}

// C++ wrapper class
class Taurine {
private:
    TaurineVM* vm;

public:
    Taurine() : vm(taurine_new()) {
        if (!vm) {
            throw std::runtime_error("Failed to create VM");
        }
    }

    ~Taurine() {
        taurine_free(vm);
    }

    void run(const std::string& code) {
        if (taurine_run(vm, code.c_str()) != 0) {
            throw std::runtime_error(taurine_get_error(vm));
        }
    }
};

int main() {
    try {
        std::cout << "=== C++ API Embedding Example ===\n\n";

        Taurine taurine;

        // Run Taurine code
        std::cout << "Running Taurine code...\n\n";
        taurine.run(R"(
            print("Hello from C++!")
            let x = 10
            let y = 20
            print(f"x + y = {x + y}")
        )");

        std::cout << "\n=== Success! ===\n";

    } catch (const std::exception& e) {
        std::cerr << "Error: " << e.what() << std::endl;
        return 1;
    }

    return 0;
}
