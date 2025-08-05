#include <stddef.h>

int _fltused = 0;

extern __chkstk(size_t ptr)
{
}

extern _aullrem(unsigned long long a, unsigned long long b)
{
    return a % b;
}

extern _aulldiv(unsigned long long a, unsigned long long b)
{
    return a / b;
}