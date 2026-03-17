/* examples/embedding/04_c_api.c
 * Example: Using Taurine from C
 *
 * Note: This is a demonstration of the C API structure.
 * The full C API implementation is in src/ffi.rs
 *
 * Compile with:
 *   gcc -o 04_c_api 04_c_api.c -ltaurine
 */

#include <stdio.h>

// Forward declarations (would be in taurine.h)
typedef struct TaurineVM TaurineVM;

extern TaurineVM* taurine_new(void);
extern void taurine_free(TaurineVM* vm);
extern int taurine_run(TaurineVM* vm, const char* code);
extern const char* taurine_get_error(TaurineVM* vm);

int main() {
    printf("=== C API Embedding Example ===\n\n");

    // Create VM
    TaurineVM* vm = taurine_new();
    if (!vm) {
        fprintf(stderr, "Failed to create VM\n");
        return 1;
    }

    // Run Taurine code
    const char* code =
        "print(\"Hello from C!\")\n"
        "let x = 42\n"
        "print(f\"x = {x}\")\n";

    printf("Running Taurine code...\n\n");

    if (taurine_run(vm, code) != 0) {
        fprintf(stderr, "Error: %s\n", taurine_get_error(vm));
        taurine_free(vm);
        return 1;
    }

    // Clean up
    taurine_free(vm);

    printf("\n=== Success! ===\n");

    return 0;
}
