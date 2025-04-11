#include <stddef.h>

int _fltused = 0;

extern __chkstk(size_t ptr) {
}

// The following is borrowed from minwindef.h
typedef unsigned short WORD;

typedef unsigned __int64 ULONG_PTR, *PULONG_PTR;
typedef ULONG_PTR DWORD_PTR, *PDWORD_PTR;

#define LOWORD(l) ((WORD) (((DWORD_PTR) (l)) & 0xffff))
#define HIWORD(l) ((WORD) ((((DWORD_PTR) (l)) >> 16) & 0xffff))

WORD loword(size_t ptr) {
    return LOWORD(ptr);
}

WORD hiword(size_t ptr) {
    return HIWORD(ptr);
}