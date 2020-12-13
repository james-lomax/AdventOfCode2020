#pragma once

#include <stdint.h>
#include <stdlib.h>

typedef struct 
{
    uint64_t* d;
    size_t len;
    size_t cap;
} IntList_t;

IntList_t ilist_alloc();
void ilist_free(IntList_t* il);
/** In place slice (no copying) */
IntList_t ilist_slice(IntList_t il, size_t offs, size_t len);
void ilist_push(IntList_t* il, uint64_t n);
