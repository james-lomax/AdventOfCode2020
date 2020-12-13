#include "intlist.h"

#include <assert.h>

IntList_t ilist_alloc()
{
    IntList_t i;
    i.d = (uint64_t*)malloc(sizeof(uint64_t)*4);
    i.len = 0;
    i.cap = 4;
    return i;
}

void ilist_free(IntList_t* il)
{
    free(il->d);
    il->d = NULL;
    il->cap = 0;
    il->len = 0;
}

/** In place slice (no copying) */
IntList_t ilist_slice(IntList_t il, size_t offs, size_t len)
{
    assert(offs + len <= il.len);
    IntList_t i;
    i.d = il.d + offs;
    i.len = len;
    i.cap = il.cap - offs;

    return i;
}

void ilist_push(IntList_t* il, uint64_t n)
{
    assert(il->d != NULL);
    if (il->len == il->cap)
    {
        size_t newcap = il->cap*2;
        uint64_t* newd = (uint64_t*)malloc(sizeof(uint64_t)*newcap);
        memcpy(newd, il->d, sizeof(uint64_t)*il->len);
        free(il->d);
        il->d = newd;
        il->cap = newcap;
    }

    il->d[il->len++] = n;
}
