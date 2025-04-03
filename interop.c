#include <Windows.h>

int load_string(LPWSTR buffer, UINT id) {
    return LoadStringW(
        GetModuleHandleA(NULL),
        id,
        buffer,
        0
    );
}